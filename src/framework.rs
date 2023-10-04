use self::implementation::prepare;
use self::{benchmark_options::BenchmarkOptions, implementation::prepare_with_id};
use crate::{
    dataset,
    implementations::{
        jsonpath_rust::{JsonpathRust, JsonpathRustError},
        rsonpath::{Rsonpath, RsonpathError, RsonpathMmap},
        rust_jsurfer::{JSurfer, JSurferError},
        serde_json_path::{SerdeJsonPath, SerdeJsonPathError},
    },
};
use criterion::{Criterion, Throughput};
use implementation::{Implementation, PreparedQuery};
use std::{path::PathBuf, time::Duration};
use thiserror::Error;

pub mod benchmark_options;
pub mod implementation;

#[derive(Clone, Copy, Debug)]
pub enum BenchTarget<'q> {
    RsonpathMmap(&'q str),
    Rsonpath(&'q str),
    JSurfer(&'q str),
    JsonpathRust(&'q str),
    SerdeJsonPath(&'q str),
}

pub struct Benchset {
    id: String,
    options: BenchmarkOptions,
    json_document: dataset::JsonFile,
    implementations: Vec<Box<dyn BenchFn>>,
    measure_file_load: bool,
}

pub struct ConfiguredBenchset {
    source: Benchset,
}

impl ConfiguredBenchset {
    pub fn run(&self, c: &mut Criterion) {
        let bench = &self.source;
        let mut group = c.benchmark_group(&bench.id);

        bench.options.apply_to(&mut group);
        group.throughput(Throughput::BytesDecimal(
            u64::try_from(bench.json_document.size_in_bytes).unwrap(),
        ));

        for implementation in bench.implementations.iter() {
            let id = implementation.id();
            group.bench_function(id, |b| b.iter(move || implementation.run()));
        }

        group.finish();
    }
}

impl Benchset {
    pub fn new<S: Into<String>>(id: S, dataset: dataset::Dataset) -> Result<Self, BenchmarkError> {
        let json_file = dataset.file_path().map_err(BenchmarkError::DatasetError)?;

        let warm_up_time = if json_file.size_in_bytes < 10_000_000 {
            None
        } else if json_file.size_in_bytes < 100_000_000 {
            Some(Duration::from_secs(5))
        } else {
            Some(Duration::from_secs(10))
        };

        // We're aiming for over 1GB/s, but some queries run at 100MB/s.
        // Let's say we want to run the query at least 10 times to get significant results.
        const TARGET_NUMBER_OF_QUERIES: f64 = 10.0;
        const TARGET_SPEED_IN_BYTES_PER_SEC: f64 = 100_000_000.0;

        let measurement_secs =
            (json_file.size_in_bytes as f64) * TARGET_NUMBER_OF_QUERIES / TARGET_SPEED_IN_BYTES_PER_SEC;
        let measurement_time = if measurement_secs > 5.0 {
            Some(Duration::from_secs_f64(measurement_secs))
        } else {
            None
        };
        let sample_count = if json_file.size_in_bytes < 100_000_000 {
            None
        } else {
            Some(10)
        };

        Ok(Self {
            id: format!("{}_{}", json_file.file_path, id.into()),
            options: BenchmarkOptions {
                warm_up_time,
                measurement_time,
                sample_count,
            },
            json_document: json_file,
            implementations: vec![],
            measure_file_load: true,
        })
    }

    pub fn do_not_measure_file_load_time(self) -> Self {
        Self {
            measure_file_load: false,
            ..self
        }
    }

    pub fn add_target(mut self, target: BenchTarget<'_>) -> Result<Self, BenchmarkError> {
        let bench_fn = target.to_bench_fn(&self.json_document.file_path, !self.measure_file_load)?;
        self.implementations.push(bench_fn);
        Ok(self)
    }

    pub fn add_target_with_id(mut self, target: BenchTarget<'_>, id: &'static str) -> Result<Self, BenchmarkError> {
        let bench_fn = target.to_bench_fn_with_id(&self.json_document.file_path, !self.measure_file_load, id)?;
        self.implementations.push(bench_fn);
        Ok(self)
    }

    pub fn add_all_targets_except_jsurfer(self, query: &str) -> Result<Self, BenchmarkError> {
        self.add_target(BenchTarget::RsonpathMmap(query))?
            .add_target(BenchTarget::JsonpathRust(query))?
            .add_target(BenchTarget::SerdeJsonPath(query))
    }

    pub fn add_all_targets(self, query: &str) -> Result<Self, BenchmarkError> {
        self.add_target(BenchTarget::RsonpathMmap(query))?
            .add_target(BenchTarget::JSurfer(query))?
            .add_target(BenchTarget::JsonpathRust(query))?
            .add_target(BenchTarget::SerdeJsonPath(query))
    }

    pub fn add_rust_native_targets(self, query: &str) -> Result<Self, BenchmarkError> {
        self.add_target(BenchTarget::RsonpathMmap(query))?
            .add_target(BenchTarget::JsonpathRust(query))?
            .add_target(BenchTarget::SerdeJsonPath(query))
    }

    pub fn finish(self) -> ConfiguredBenchset {
        ConfiguredBenchset { source: self }
    }
}

trait Target {
    fn to_bench_fn(self, file_path: &str, load_ahead_of_time: bool) -> Result<Box<dyn BenchFn>, BenchmarkError>;

    fn to_bench_fn_with_id(
        self,
        file_path: &str,
        load_ahead_of_time: bool,
        id: &'static str,
    ) -> Result<Box<dyn BenchFn>, BenchmarkError>;
}

impl<'a> Target for BenchTarget<'a> {
    fn to_bench_fn(self, file_path: &str, load_ahead_of_time: bool) -> Result<Box<dyn BenchFn>, BenchmarkError> {
        match self {
            BenchTarget::Rsonpath(q) => {
                let rsonpath = Rsonpath::new()?;
                let prepared = prepare(rsonpath, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::RsonpathMmap(q) => {
                let rsonpath = RsonpathMmap::new()?;
                let prepared = prepare(rsonpath, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JSurfer(q) => {
                let jsurfer = JSurfer::new()?;
                let prepared = prepare(jsurfer, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JsonpathRust(q) => {
                let jsonpath_rust = JsonpathRust::new()?;
                let prepared = prepare(jsonpath_rust, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::SerdeJsonPath(q) => {
                let serde_json_path = SerdeJsonPath::new()?;
                let prepared = prepare(serde_json_path, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
        }
    }

    fn to_bench_fn_with_id(
        self,
        file_path: &str,
        load_ahead_of_time: bool,
        id: &'static str,
    ) -> Result<Box<dyn BenchFn>, BenchmarkError> {
        match self {
            BenchTarget::Rsonpath(q) => {
                let rsonpath = Rsonpath::new()?;
                let prepared = prepare_with_id(rsonpath, id, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::RsonpathMmap(q) => {
                let rsonpath = RsonpathMmap::new()?;
                let prepared = prepare_with_id(rsonpath, id, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JSurfer(q) => {
                let jsurfer = JSurfer::new()?;
                let prepared = prepare_with_id(jsurfer, id, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::JsonpathRust(q) => {
                let jsonpath_rust = JsonpathRust::new()?;
                let prepared = prepare_with_id(jsonpath_rust, id, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
            BenchTarget::SerdeJsonPath(q) => {
                let serde_json_path = SerdeJsonPath::new()?;
                let prepared = prepare_with_id(serde_json_path, id, file_path, q, load_ahead_of_time)?;
                Ok(Box::new(prepared))
            }
        }
    }
}

trait BenchFn {
    fn id(&self) -> &str;

    fn run(&self);
}

impl<I: Implementation> BenchFn for PreparedQuery<I> {
    fn id(&self) -> &str {
        self.id
    }

    fn run(&self) {
        match &self.file {
            implementation::File::NeedToLoad(file_path) => {
                let f = self.implementation.load_file(file_path).unwrap();
                let result = self.implementation.run(&self.query, &f).unwrap();
                criterion::black_box(result);
            }
            implementation::File::AlreadyLoaded(f) => {
                let result = self.implementation.run(&self.query, f).unwrap();
                criterion::black_box(result);
            }
        };
    }
}

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("invalid dataset file path, has to be valid UTF-8: '{0}'")]
    InvalidFilePath(PathBuf),
    #[error("error loading dataset: {0}")]
    DatasetError(
        #[source]
        #[from]
        dataset::DatasetError,
    ),
    #[error("error preparing Rsonpath bench: {0}")]
    RsonpathError(
        #[source]
        #[from]
        RsonpathError,
    ),
    #[error("error preparing JSurfer bench: {0}")]
    JSurferError(
        #[source]
        #[from]
        JSurferError,
    ),
    #[error("error preparing JsonpathRust bench: {0}")]
    JsonpathRust(
        #[source]
        #[from]
        JsonpathRustError,
    ),
    #[error("error preparing SerdeJsonPath bench: {0}")]
    SerdeJsonPath(
        #[source]
        #[from]
        SerdeJsonPathError,
    ),
}
