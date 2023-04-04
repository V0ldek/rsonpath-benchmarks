use rsonpath_benchmarks::prelude::*;

pub fn ast_nested_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::nested_inner", dataset::ast())?
        .add_target(BenchTarget::Rsonpath("$..inner..inner..type.qualType"))?
        .add_target(BenchTarget::JsonpathRust("$..inner..inner..type.qualType"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_category", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::Rsonpath("$.products[*].categoryPath[*].id"))?
        .add_target(BenchTarget::JsonpathRust(
            "$.products[*].categoryPath[*].id",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new(
        "bestbuy::products_video_only",
        dataset::pison_bestbuy_large(),
    )?
    .add_target_with_id(
        BenchTarget::Rsonpath("$.products[*].videoChapters"),
        "rsonpath_direct",
    )?
    .add_target_with_id(
        BenchTarget::Rsonpath("$..videoChapters"),
        "rsonpath_descendant",
    )?
    .add_target_with_id(
        BenchTarget::JsonpathRust("$..videoChapters"),
        "jsonpath-rust_direct",
    )?
    .add_target_with_id(
        BenchTarget::JsonpathRust("$.products[*].videoChapters"),
        "jsonpath-rust_descendant",
    )?
    .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::routes", dataset::pison_google_map())?
        .add_target(BenchTarget::Rsonpath(
            "$[*].routes[*].legs[*].steps[*].distance.text",
        ))?
        .add_target(BenchTarget::JsonpathRust(
            "$[*].routes[*].legs[*].steps[*].distance.text",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes", dataset::pison_google_map())?
        .add_target_with_id(
            BenchTarget::Rsonpath("$[*].available_travel_modes"),
            "rsonpath_direct",
        )?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..available_travel_modes"),
            "rsonpath_descendant",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$[*].available_travel_modes"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$..available_travel_modes"),
            "jsonpath-rust_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("walmart::items_name", dataset::pison_walmart())?
        .add_target_with_id(BenchTarget::Rsonpath("$.items[*].name"), "rsonpath_direct")?
        .add_target_with_id(
            BenchTarget::Rsonpath("$..items_name"),
            "rsonpath_descendant",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$.items[*].name"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$..items_name"),
            "jsonpath-rust_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata", dataset::twitter())?
        .add_target_with_id(
            BenchTarget::Rsonpath("$.search_metadata.count"),
            "rsonpath_direct",
        )?
        .add_target_with_id(BenchTarget::JsonpathRust("$..count"), "rsonpath_descendant")?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$.search_metadata.count"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$..count"),
            "jsonpath-rust_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    ast_nested_inner,
    bestbuy_products_category,
    bestbuy_products_video_only,
    google_map_routes,
    google_map_travel_modes,
    walmart_items_name,
    twitter_metadata,
);
