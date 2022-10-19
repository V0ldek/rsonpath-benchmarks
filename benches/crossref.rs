use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rsonpath::engine::result::CountResult;
use rsonpath::engine::{Input, Runner};
use rsonpath::query::JsonPathQuery;
use rsonpath::stackless::StacklessRunner;
use rsonpath_benchmarks::rust_jsurfer;
use rsonpath_benchmarks::rust_jsonski;

use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";
fn get_jsonski_record(test_path: &str) -> rust_jsonski::JsonSkiRecord {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    rust_jsonski::load_jsonski_record(&path)
}
fn get_path(test_path: &str) -> String {
    format!("{}/{}", ROOT_TEST_DIRECTORY, test_path)
}

fn get_contents(test_path: &str) -> Input {
    let raw = fs::read_to_string(get_path(test_path)).unwrap();
    Input::new(raw)
}

struct BenchmarkOptions<'a> {
    pub query_string: &'a str,
    pub jsonski_query_string: &'a str,
    pub id: &'a str,
    pub warm_up_time: Duration,
    pub measurement_time: Duration,
}

fn crossref(c: &mut Criterion, options: BenchmarkOptions<'_>, i: u8, include_jsurf: bool) {
    let data_path : &str = &format!("crossref/crossref{}.json", i);
    let contents = get_contents(data_path);
    let query = JsonPathQuery::parse(options.query_string).unwrap();

    let mut group = c.benchmark_group(format! {"crossref_{}", options.id});
    group.warm_up_time(options.warm_up_time);
    group.measurement_time(options.measurement_time);
    group.throughput(criterion::Throughput::BytesDecimal(contents.len() as u64));

    let rsonpath = StacklessRunner::compile_query(&query);

    group.bench_with_input(
        BenchmarkId::new("rsonpath", options.query_string),
        &contents,
        |b, c| b.iter(|| rsonpath.run::<CountResult>(c)),
    );
    if include_jsurf{
        let context = rust_jsurfer::Jvm::attach().expect("failed to attach to Jvm");
        let jsurfer_file = context
            .load_file(&get_path(data_path))
            .expect("failed to load file via jsurfer");
        let jsurfer_query = context
            .compile_query(options.query_string)
            .expect("failed to compile query via jsurfer");
        group.bench_with_input(
            BenchmarkId::new("jsurfer", options.query_string),
            &(&jsurfer_file, &jsurfer_query),
            |b, &(f, q)| {
                b.iter(|| q.run(f).unwrap());
            },
    );
    }
    if !options.jsonski_query_string.is_empty() {
        let jsonski_query = rust_jsonski::create_jsonski_query(options.jsonski_query_string);
        let jsonski_record = get_jsonski_record(data_path);
        group.bench_with_input(
            BenchmarkId::new("jsonski", options.jsonski_query_string),
            &(&jsonski_record, &jsonski_query),
            |b, &(r, q)| {
                b.iter(|| rust_jsonski::call_jsonski(q, r));
            },
        );
    }
    group.finish();
}
pub fn doi(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..DOI",
            jsonski_query_string: "",
            id: "DOI",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        8,
        true,

    )
}
pub fn title(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..title",
            jsonski_query_string: "$.items[*].title",
            id: "title",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        8,
        true,
    )
}
pub fn orcid(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..author..ORCID",
            jsonski_query_string: "$.items[*].author[*].ORCID",
            id: "orcid",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        8,
        true,
    )
}

pub fn affiliation(c: &mut Criterion) {
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "$.items[*].author[*].affiliation[*].name",
            id: "affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        8,
        true,
    )
}

pub fn scalability0(c: &mut Criterion){
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "",
            id: "scalability0_affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        0,
        false,
    )
     
}

pub fn scalability1(c: &mut Criterion){
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "",
            id: "scalability1_affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        1,
        false,
    )
     
}

pub fn scalability2(c: &mut Criterion){
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "",
            id: "scalability2_affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        2,
        false,
    )
     
}

pub fn scalability4(c: &mut Criterion){
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "",
            id: "scalability4_affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        4,
        false,
    )
     
}

pub fn scalability8(c: &mut Criterion){
    crossref(
        c,
        BenchmarkOptions {
            query_string: "$..affiliation..name",
            jsonski_query_string: "",
            id: "scalability8_affiliation",
            warm_up_time: Duration::from_secs(10),
            measurement_time: Duration::from_secs(40),
        },
        8,
        false,
    )
     
}

criterion_group!(crossref_benches, affiliation, orcid, title, doi, scalability0, scalability1, scalability2, scalability4, scalability8);
criterion_main!(crossref_benches);
