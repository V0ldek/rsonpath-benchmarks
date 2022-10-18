use criterion::{BenchmarkGroup, Criterion, Throughput};
use implementation::{Implementation, PreparedQuery};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use self::implementation::prepare;

pub mod implementation;

struct Benchset<'a> {
    id: String,
    options: BenchmarkOptions,
    json_document: JsonDocument,
    implementations: Vec<Box<dyn BenchFn<'a>>>,
}

impl<'a> Benchset<'a> {
    pub fn run(&'a self, c: &mut Criterion) {
        let mut group = c.benchmark_group(&self.id);

        self.options.apply_to(&mut group);
        group.throughput(Throughput::BytesDecimal(self.json_document.number_of_bytes));

        for implementation in self.implementations.iter() {
            group.bench_function(&self.id, |b| b.iter(move || implementation.run()));
        }

        group.finish();
    }

    pub fn new<S: Into<String>, P: AsRef<Path>>(id: S, file_path: P) -> Self {
        let json_document = JsonDocument::new(file_path.as_ref().to_owned());

        Self {
            id: id.into(),
            options: BenchmarkOptions {
                warm_up_time: None,
                measurement_time: None,
            },
            json_document,
            implementations: vec![],
        }
    }

    pub fn add_implementation<'a, I: Implementation<'a>>(mut self, implementation: &'a I, query: &str) -> Result<Self, I::Error> {
        let implementation = I::new()?;
        let prepared = prepare(implementation, &self.json_document.file_path, query);
        
        self
    }
}

struct BenchmarkOptions {
    warm_up_time: Option<Duration>,
    measurement_time: Option<Duration>,
}

impl BenchmarkOptions {
    fn apply_to<T: criterion::measurement::Measurement>(&self, group: &mut BenchmarkGroup<T>) {
        if let Some(warm_up_time) = self.warm_up_time {
            group.warm_up_time(warm_up_time);
        }
        if let Some(measurement_time) = self.measurement_time {
            group.measurement_time(measurement_time);
        }
    }
}

struct JsonDocument {
    file_path: PathBuf,
    number_of_bytes: u64,
}

impl JsonDocument {
    fn new(file_path: PathBuf) -> Self {
        use std::fs;
        let len = fs::metadata(&file_path).unwrap().len();

        Self {
            file_path,
            number_of_bytes: len,
        }
    }
}

trait BenchFn<'a> {
    fn run(&'a self) -> u64;
}

impl<'a, I: Implementation<'a>> BenchFn<'a> for PreparedQuery<'a, I> {
    fn run(&'a self) -> u64 {
        self.implementation.run(&self.query, &self.file).unwrap()
    }
}
