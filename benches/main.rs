use rsonpath_benchmarks::prelude::*;

pub fn ast_nested_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::nested_inner", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$..inner..inner..type.qualType"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn ast_deepest(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::deepest", dataset::ast())?
    .do_not_measure_file_load_time()
        .add_target_with_id(
            BenchTarget::Rsonpath("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*"),
            "101_dalmatians")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_category", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$.products[*].categoryPath[*].id"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(BenchTarget::Rsonpath("$.products[*].videoChapters"), "rsonpath_direct")?
        .add_target_with_id(BenchTarget::Rsonpath("$..videoChapters"), "rsonpath_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_all_nodes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::all_nodes", dataset::pison_bestbuy_short())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$..*"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::routes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$[*].routes[*].legs[*].steps[*].distance.text"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes", dataset::pison_google_map_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(BenchTarget::Rsonpath("$[*].available_travel_modes"), "rsonpath_direct")?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..available_travel_modes"),
            "rsonpath_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("walmart::items_name", dataset::pison_walmart_short())?
        .do_not_measure_file_load_time()
        .add_target_with_id(BenchTarget::Rsonpath("$.items[*].name"), "rsonpath_direct")?
        .add_target_with_id(BenchTarget::Rsonpath("$..items_name"), "rsonpath_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_target_with_id(BenchTarget::Rsonpath("$.search_metadata.count"), "rsonpath_direct")?
        .add_target_with_id(BenchTarget::Rsonpath("$..count"), "rsonpath_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn inner_array(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("inner_array", dataset::ast())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$..inner[0]"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn user_second_mention_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("user_mentions_indices", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$..entities.user_mentions[1]"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn all_first_index(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("all_first_index", dataset::twitter())?
        .do_not_measure_file_load_time()
        .add_target(BenchTarget::Rsonpath("$..[0]"))?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    ast_nested_inner,
    ast_deepest,
    bestbuy_products_category,
    bestbuy_products_video_only,
    bestbuy_all_nodes,
    google_map_routes,
    google_map_travel_modes,
    inner_array,
    user_second_mention_index,
    walmart_items_name,
    twitter_metadata,
    all_first_index
);
