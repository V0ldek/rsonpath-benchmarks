use rsonpath_benchmarks::prelude::*;

pub fn tt1(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("metadata_1", dataset::twitter_jsonski())?
        .add_all_targets("$.search_metadata.count")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(twitter_benches, tt1,);
