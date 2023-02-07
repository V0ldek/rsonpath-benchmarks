use hex_literal::hex;
use reqwest::blocking as reqwest;
use sha2::{Digest, Sha256};
use std::fmt::Display;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

type Sha256Digest = [u8; 32];

pub(crate) struct JsonFile {
    pub(crate) file_path: String,
    pub(crate) size_in_bytes: usize,
    checksum: Sha256Digest,
}

pub struct Dataset {
    name: &'static str,
    path: &'static str,
    source: DatasetSource,
    checksum: Sha256Digest,
}

#[derive(Debug, Clone)]
pub enum DatasetSource {
    UrlJson(&'static str),
    UrlArchive(DatasetArchive),
    UrlTarArchive(DatasetArchive, &'static str),
}

#[derive(Debug, Clone)]
pub struct DatasetArchive {
    url: &'static str,
    checksum: Sha256Digest,
}

impl DatasetSource {
    fn url(&self) -> &'static str {
        match self {
            Self::UrlJson(url) => url,
            Self::UrlArchive(archive) | Self::UrlTarArchive(archive, _) => archive.url,
        }
    }
}

impl DatasetArchive {
    fn validate_archive_checksum(&self, actual: Sha256Digest) -> Result<(), DatasetError> {
        if self.checksum != actual {
            Err(DatasetError::InvalidArchiveChecksum(
                self.url,
                self.checksum,
                actual,
            ))
        } else {
            Ok(())
        }
    }
}

impl Dataset {
    pub(crate) fn file_path(&self) -> Result<JsonFile, DatasetError> {
        match self.load_file()? {
            Some(json_file) if self.checksum == json_file.checksum => return Ok(json_file),
            Some(json_file) => {
                eprintln!(
                    "File for dataset {} does not match expected checksum ({} expected, {} actual). Redownloading.",
                    self.name, format_hex_string(&self.checksum), format_hex_string(&json_file.checksum));
            }
            None => {
                eprintln!("File for dataset {} does not exist.", self.name);
            }
        }
        let new_json_file = self.download_file()?;

        if new_json_file.checksum != self.checksum {
            Err(DatasetError::InvalidJsonChecksum(
                self.source.url(),
                self.checksum,
                new_json_file.checksum,
            ))
        } else {
            Ok(new_json_file)
        }
    }

    fn json_path(&self) -> &Path {
        self.path.as_ref()
    }

    fn directory_path(&self) -> Result<&Path, DatasetError> {
        self.json_path()
            .parent()
            .ok_or(DatasetError::InvalidPath(self.path))
    }

    fn create_directories(&self) -> Result<(), DatasetError> {
        fs::create_dir_all(self.directory_path()?).map_err(DatasetError::FileSystemError)
    }

    fn archive_path(&self) -> PathBuf {
        self.json_path().with_extension("gz")
    }

    fn load_file(&self) -> Result<Option<JsonFile>, DatasetError> {
        match fs::File::open(self.path) {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                let progress = get_progress_bar("Checking dataset integrity...", None);
                let (md5, size_in_bytes) =
                    read_digest_and_write::<_, fs::File>(progress.wrap_read(reader), None)?;

                Ok(Some(JsonFile {
                    file_path: self.path.to_string(),
                    checksum: md5,
                    size_in_bytes,
                }))
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(DatasetError::FileSystemError(err)),
        }
    }

    fn download_file(&self) -> Result<JsonFile, DatasetError> {
        match self.source {
            DatasetSource::UrlJson(url) => self.download_json(url),
            DatasetSource::UrlArchive(ref archive) => self.download_archive(archive),
            DatasetSource::UrlTarArchive(ref archive, initial_path) => {
                self.download_tar_archive(archive, initial_path.as_ref())
            }
        }
    }

    fn download_json(&self, url: &'static str) -> Result<JsonFile, DatasetError> {
        self.create_directories()?;
        let mut file = fs::File::create(self.json_path()).map_err(DatasetError::FileSystemError)?;

        let response = make_download_request(url)?;
        let progress = get_progress_bar("Downloading", response.content_length());
        let (md5, size_in_bytes) =
            read_digest_and_write(progress.wrap_read(response), Some(&mut file))?;
        progress.finish_and_clear();

        Ok(JsonFile {
            file_path: self.path.to_string(),
            checksum: md5,
            size_in_bytes,
        })
    }

    fn download_archive(&self, archive: &DatasetArchive) -> Result<JsonFile, DatasetError> {
        use flate2::read::GzDecoder;

        self.create_directories()?;
        let archive_path = self.archive_path();
        let mut archive_file =
            fs::File::create(&archive_path).map_err(DatasetError::FileSystemError)?;

        let response = make_download_request(archive.url)?;
        let progress = get_progress_bar("Downloading", response.content_length());
        let (checksum, archive_size) =
            read_digest_and_write(progress.wrap_read(response), Some(&mut archive_file))?;
        progress.finish_and_clear();
        archive_file
            .flush()
            .map_err(DatasetError::InputOutputError)?;

        archive.validate_archive_checksum(checksum)?;

        let mut json_file =
            fs::File::create(self.json_path()).map_err(DatasetError::FileSystemError)?;
        let archive_file = fs::File::open(archive_path).map_err(DatasetError::FileSystemError)?;
        let progress = get_progress_bar("Extracting", Some(archive_size as u64));
        let gz = GzDecoder::new(progress.wrap_read(archive_file));
        let (md5, size_in_bytes) = read_digest_and_write(gz, Some(&mut json_file))?;
        progress.finish_and_clear();

        Ok(JsonFile {
            file_path: self.path.to_string(),
            checksum: md5,
            size_in_bytes,
        })
    }

    fn download_tar_archive(
        &self,
        archive: &DatasetArchive,
        initial_path: &Path,
    ) -> Result<JsonFile, DatasetError> {
        self.create_directories()?;
        let archive_path = self.archive_path();
        let mut archive_file =
            fs::File::create(&archive_path).map_err(DatasetError::FileSystemError)?;

        let response = make_download_request(archive.url)?;
        let progress = get_progress_bar("Downloading", response.content_length());
        let (checksum, archive_size) =
            read_digest_and_write(progress.wrap_read(response), Some(&mut archive_file))?;
        progress.finish_and_clear();
        archive_file
            .flush()
            .map_err(DatasetError::InputOutputError)?;

        archive.validate_archive_checksum(checksum)?;

        unpack_tar_gz(&archive_path, archive_size, initial_path)?;

        let json_file = fs::File::open(self.json_path()).map_err(DatasetError::FileSystemError)?;
        let (md5, size_in_bytes) = read_digest_and_write::<fs::File, fs::File>(json_file, None)?;

        Ok(JsonFile {
            file_path: self.path.to_string(),
            checksum: md5,
            size_in_bytes,
        })
    }
}

fn unpack_tar_gz(
    archive_path: &Path,
    archive_size: usize,
    target_path: &Path,
) -> Result<(), DatasetError> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let archive_file = fs::File::open(archive_path).map_err(DatasetError::FileSystemError)?;
    let progress =
        get_progress_bar("Extracting", Some(archive_size as u64)).wrap_read(archive_file);
    let gz = GzDecoder::new(progress);
    let mut tar = Archive::new(gz);
    tar.unpack(target_path)
        .map_err(DatasetError::InputOutputError)
}

fn make_download_request(url: &'static str) -> Result<reqwest::Response, DatasetError> {
    use std::time::Duration;
    let msg = format!("Downloading {url}");
    let progress = get_progress_bar(msg, None);
    progress.enable_steady_tick(Duration::from_millis(83));
    let response = reqwest::get(url).map_err(|err| DatasetError::DownloadError(url, err))?;
    progress.finish();
    Ok(response)
}

fn get_progress_bar<S>(msg: S, content: Option<u64>) -> indicatif::ProgressBar
where
    S: Into<std::borrow::Cow<'static, str>>,
{
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(
        "{msg} {spinner} {wide_bar:.green/white} {bytes:>12}/{total_bytes:>12} ({bytes_per_sec:>12}) {eta:>10}"
    ).unwrap().progress_chars("=>-");
    let progress = content.map_or_else(ProgressBar::new_spinner, |x| {
        ProgressBar::new(x).with_style(style)
    });
    progress.set_message(msg);

    progress
}

fn read_digest_and_write<R, W>(
    mut reader: R,
    mut writer: Option<&mut W>,
) -> Result<(Sha256Digest, usize), DatasetError>
where
    R: Read,
    W: Write,
{
    let mut total_size = 0;
    let mut buf = [0; 65_536];
    let mut hasher = Sha256::new();
    loop {
        let size = reader
            .read(&mut buf)
            .map_err(DatasetError::InputOutputError)?;
        if size == 0 {
            break;
        }
        total_size += size;
        hasher.update(&buf[..size]);

        if let Some(w) = writer.as_mut() {
            w.write_all(&buf[..size])
                .map_err(DatasetError::InputOutputError)?;
        }
    }

    Ok((hasher.finalize().into(), total_size))
}

macro_rules! dataset_path {
    ($e:expr) => {
        concat! {"./data", "/", $e}
    };
}

pub const fn ast() -> Dataset {
    Dataset {
        name: "ast",
        path: dataset_path!("ast/ast.json"),
        source: DatasetSource::UrlJson("https://zenodo.org/record/7229269/files/ast.json"),
        checksum: hex!("c3ff840d153953ee08c1d9622b20f8c1dc367ae2abcb9c85d44100c6209571af"),
    }
}

pub fn crossref(size: u32) -> Dataset {
    let source = DatasetSource::UrlTarArchive(
        DatasetArchive {
            url: "https://zenodo.org/record/7343312/files/crossref.tar.gz",
            checksum: hex!("eddb87d1cf7490974236c3ba68a0e4237189aec4b9c27befd020d6e24d45c1db"),
        },
        dataset_path!(""),
    );

    match size {
        0 => Dataset {
            name: "crossref0",
            path: dataset_path!("crossref/crossref0.json"),
            source,
            checksum: hex!("db314fb19b527d5aa4e0e7d2b05c45d183af0f0aed8af285ce20c044e9789943"),
        },
        1 => Dataset {
            name: "crossref1",
            path: dataset_path!("crossref/crossref1.json"),
            source,
            checksum: hex!("723527cbf9b642cb7cb63cd877496f72115a76a36b4c86814f2776d6950fcc48"),
        },
        2 => Dataset {
            name: "crossref2",
            path: dataset_path!("crossref/crossref2.json"),
            source,
            checksum: hex!("6c452a0ee33a0fc9c98e6830e6fb411e3f4736507977c0e96ec3027488b4c95f"),
        },
        4 => Dataset {
            name: "crossref4",
            path: dataset_path!("crossref/crossref4.json"),
            source,
            checksum: hex!("d47b65922745e8ac02d52483692682fc79de56f13d1c7a5cd4c98237f6c394e9"),
        },
        _ => panic!("unsupported dataset crossref{size}"),
    }
}

pub const fn openfood() -> Dataset {
    Dataset {
        name: "openfood",
        path: dataset_path!("openfood/openfood.json"),
        source: DatasetSource::UrlJson("https://zenodo.org/record/7305505/files/openfood.json"),
        checksum: hex!("57ece15eecf3bbdc4d18a1215a7c3b9d0d58df0505dc4517b103dc75fac4843f"),
    }
}

pub const fn twitter() -> Dataset {
    Dataset {
        name: "twitter",
        path: dataset_path!("twitter/twitter.json"),
        source: DatasetSource::UrlJson("https://zenodo.org/record/7229287/files/twitter.json"),
        checksum: hex!("f14e65d4f8df3c9144748191c1e9d46a030067af86d0cc03cc67f22149143c5d"),
    }
}

pub const fn pison_bestbuy_large() -> Dataset {
    Dataset {
        name: "pison_bestbuy",
        path: dataset_path!("pison/bestbuy_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607865/files/bestbuy_large_record.json.gz",
            checksum: hex!("c8d5efe683256e1530922b7d198fd33c2c8764a594b04b6e8bd29346b09cfb3e"),
        }),
        checksum: hex!("8eee3043d6d0a11cecb43e169f70fae83c68efa7fe4a5508aa2192f717c45617"),
    }
}

pub const fn pison_google_map() -> Dataset {
    Dataset {
        name: "pison_google_map",
        path: dataset_path!("pison/google_map_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607889/files/google_map_large_record.json.gz",
            checksum: hex!("bff82147ec42186a016615e888c1e009f306ab0599db20afdf102cb95e6f6e5b"),
        }),
        checksum: hex!("cdbc090edf4faeea80d917e3a2ff618fb0a42626eeac5a4521dae471e4f53574"),
    }
}

pub const fn pison_nspl() -> Dataset {
    Dataset {
        name: "pison_nspl",
        path: dataset_path!("pison/nspl_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607878/files/nspl_large_record.json.gz",
            checksum: hex!("9faccd67b68afd1e750af007093a42cebe876af2143d5954f1607aa8b05479a5"),
        }),
        checksum: hex!("174978fd3d7692dbf641c00c80b34e3ff81f0d3d4602c89ee231b989e6a30dd3"),
    }
}

pub const fn pison_twitter() -> Dataset {
    Dataset {
        name: "pison_twitter",
        path: dataset_path!("pison/twitter_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607891/files/twitter_large_record.json.gz",
            checksum: hex!("4e8bfb5e68bd1b4a9c69c7f2515eb65608ce84e3c284ecb1fe6908eb57b4e650"),
        }),
        checksum: hex!("2357e2bdba1d621a20c2278a88bdec592e93c680de17d8403d9e3018c7539da6"),
    }
}

pub const fn pison_walmart() -> Dataset {
    Dataset {
        name: "pison_walmart",
        path: dataset_path!("pison/walmart_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607882/files/walmart_large_record.json.gz",
            checksum: hex!("3ba4309dd620463045a3996596805f738ead2b257cf7152ea6b1f8ab339e71f4"),
        }),
        checksum: hex!("ebad2cf96871a1c2277c2a19dcc5818f9c2aed063bc8a56459f378024c5a6e14"),
    }
}

pub const fn pison_wiki() -> Dataset {
    Dataset {
        name: "pison_wiki",
        path: dataset_path!("pison/wiki_large_record.json"),
        source: DatasetSource::UrlArchive(DatasetArchive {
            url: "https://zenodo.org/record/7607884/files/wiki_large_record.json.gz",
            checksum: hex!("60755f971307f29cebbb7daa8624acec41c257dfef5c1543ca0934f5b07edcf7"),
        }),
        checksum: hex!("1abea7979812edc38651a631b11faf64f1eb5a61e2ee875b4e4d4f7b15a8cea9"),
    }
}

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("Filesystem error: {0}")]
    FileSystemError(#[source] std::io::Error),
    #[error("I/O error reading dataset contents: {0}")]
    InputOutputError(#[source] std::io::Error),
    #[error("Invalid dataset path: {0} is not a valid path")]
    InvalidPath(&'static str),
    #[error("Error downloading a dataset from {0}: {1}")]
    DownloadError(&'static str, #[source] ::reqwest::Error),
    #[error(
        "Checksum validation failed. \
    The URL source might be corrupted. \
    Expected JSON from {0} to have SHA2 checksum of {}, but it has {}.", format_hex_string(.1), format_hex_string(.2)
    )]
    InvalidJsonChecksum(&'static str, Sha256Digest, Sha256Digest),
    #[error(
        "Checksum validation failed. \
    The URL source might be corrupted. \
    Expected archive from {0} to have SHA2 checksum of {}, but it has {}.", format_hex_string(.1), format_hex_string(.2)
    )]
    InvalidArchiveChecksum(&'static str, Sha256Digest, Sha256Digest),
}

fn format_hex_string(bytes: &[u8]) -> impl Display {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
}
