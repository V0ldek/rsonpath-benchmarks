use rsonpath_benchmarks::prelude::*;

fn az_tenants_ids(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("az_tenants::ids", dataset::az_tenants())?
        .do_not_measure_file_load_time()
        .measure_compilation_time()
        .add_rsonpath_with_all_result_types("$[*].tenantId")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(main_micro_benches, az_tenants_ids,);
