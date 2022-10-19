use crate::{
    rsonpath::{Rsonpath, RsonpathError},
    rust_jsonski::{JsonSki, JsonSkiError},
    rust_jsurfer::{JSurfer, JSurferError},
};

use self::benchmark_options::BenchmarkOptions;
use self::implementation::prepare;
use self::json_document::JsonDocument;
use criterion::{Criterion, Throughput};
use implementation::{Implementation, PreparedQuery};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod benchmark_options;
pub mod implementation;
pub mod json_document;

#[derive(Clone, Copy, Debug)]
pub enum BenchTarget<'q> {
    Rsonpath(&'q str),
    JsonSki(&'q str),
    JSurfer(&'q str),
}

pub struct Benchset {
    id: String,
    options: BenchmarkOptions,
    json_document: JsonDocument,
    implementations: Vec<Box<dyn BenchFn>>,
}

pub struct ConfiguredBenchset {
    source: Benchset
}

impl ConfiguredBenchset {
    pub fn run(&self, c: &mut Criterion) {
        let bench = &self.source;
        let mut group = c.benchmark_group(&bench.id);

        bench.options.apply_to(&mut group);
        group.throughput(Throughput::BytesDecimal(bench.json_document.size_in_bytes));

        for implementation in bench.implementations.iter() {
            group.bench_function(&bench.id, |b| b.iter(move || implementation.run()));
        }

        group.finish();
    }
}

impl Benchset {
    pub fn new<S: Into<String>, P: AsRef<Path>>(
        id: S,
        file_path: P,
    ) -> Result<Self, BenchmarkError> {
        let file_path_str = file_path
            .as_ref()
            .to_str()
            .ok_or_else(|| BenchmarkError::InvalidFilePath(file_path.as_ref().to_owned()))?;
        let json_document = JsonDocument::new(file_path_str.to_owned())?;

        Ok(Self {
            id: id.into(),
            options: BenchmarkOptions {
                warm_up_time: None,
                measurement_time: None,
            },
            json_document,
            implementations: vec![],
        })
    }

    pub fn add_target(mut self, target: BenchTarget<'_>) -> Result<Self, BenchmarkError> {
        let bench_fn = target.to_bench_fn(&self.json_document.file_path)?;
        self.implementations.push(bench_fn);
        Ok(self)
    }

    pub fn finish(self) -> ConfiguredBenchset {
        ConfiguredBenchset { source: self }
    }
}

trait Target {
    fn to_bench_fn(self, file_path: &str) -> Result<Box<dyn BenchFn>, BenchmarkError>;
}

impl<'a> Target for BenchTarget<'a> {
    fn to_bench_fn(self, file_path: &str) -> Result<Box<dyn BenchFn>, BenchmarkError> {
        match self {
            BenchTarget::Rsonpath(q) => {
                let rsonpath = Rsonpath::new()?;
                let prepared = prepare(rsonpath, file_path, q)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JsonSki(q) => {
                let jsonski = JsonSki::new()?;
                let prepared = prepare(jsonski, file_path, q)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JSurfer(q) => {
                let jsurfer = JSurfer::new()?;
                let prepared = prepare(jsurfer, file_path, q)?;
                Ok(Box::new(prepared))
            }
        }
    }
}

trait BenchFn {
    fn run(&self) -> u64;
}

impl<I: Implementation> BenchFn for PreparedQuery<I> {
    fn run(&self) -> u64 {
        self.implementation.run(&self.query, &self.file).unwrap()
    }
}

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("invalid dataset file path, has to be valid UTF-8: '{0}'")]
    InvalidFilePath(PathBuf),
    #[error("error preparing Rsonpath bench")]
    RsonpathError(#[from] RsonpathError),
    #[error("error preparing JsonSki bench")]
    JsonSkiError(#[from] JsonSkiError),
    #[error("error preparing JSurfer bench")]
    JSurferError(#[from] JSurferError)
}
