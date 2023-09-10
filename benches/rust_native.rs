use rsonpath_benchmarks::prelude::*;

pub fn ast_decl_inner(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("ast::decl_inner", dataset::ast())?
        .add_rust_native_targets("$..decl.name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("bestbuy::products_video_only", dataset::pison_bestbuy_short())?
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$.products[*].videoChapters"),
            "rsonpath_direct",
        )?
        .add_target_with_id(BenchTarget::RsonpathMmap("$..videoChapters"), "rsonpath_descendant")?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$.products[*].videoChapters"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$..videoChapters"),
            "jsonpath-rust_descendant",
        )?
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$.products[*].videoChapters"),
            "serde_json_path_direct",
        )?
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$..videoChapters"),
            "serde_json_path_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("google_map::travel_modes", dataset::pison_google_map_short())?
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$[*].available_travel_modes"),
            "rsonpath_direct",
        )?
        .add_target_with_id(
            BenchTarget::RsonpathMmap("$..available_travel_modes"),
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
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$[*].available_travel_modes"),
            "serde_json_path_direct",
        )?
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$..available_travel_modes"),
            "serde_json_path_descendant",
        )?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_metadata(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("twitter::metadata", dataset::twitter())?
        .add_target_with_id(BenchTarget::RsonpathMmap("$.search_metadata.count"), "rsonpath_direct")?
        .add_target_with_id(BenchTarget::RsonpathMmap("$..count"), "rsonpath_descendant")?
        .add_target_with_id(
            BenchTarget::JsonpathRust("$.search_metadata.count"),
            "jsonpath-rust_direct",
        )?
        .add_target_with_id(BenchTarget::JsonpathRust("$..count"), "jsonpath-rust_descendant")?
        .add_target_with_id(
            BenchTarget::SerdeJsonPath("$.search_metadata.count"),
            "serde_json_path_direct",
        )?
        .add_target_with_id(BenchTarget::SerdeJsonPath("$..count"), "serde_json_path_descendant")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    main_benches,
    ast_decl_inner,
    bestbuy_products_video_only,
    google_map_travel_modes,
    twitter_metadata
);
