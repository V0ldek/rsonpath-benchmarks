use rsonpath_benchmarks::prelude::*;

pub fn bestbuy_products_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB1'_products_category", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::Rsonpath("$..categoryPath..id"))?
        .add_target(BenchTarget::JsonSki("$.products[*].categoryPath[*].id"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB2'_products_video", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::Rsonpath(
            "$..videoChapters..chapter",
        ))?
        .add_target(BenchTarget::JsonSki(
            "$.products[*].videoChapters[*].chapter",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB3'_products_video_only", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::Rsonpath("$..videoChapters"))?
        .add_target(BenchTarget::JsonSki("$.products[*].videoChapters"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("GMD2'_travel_modes", dataset::pison_google_map())?
        .add_target(BenchTarget::Rsonpath("$..available_travel_modes"))?
        .add_target(BenchTarget::JsonSki("$[*].available_travel_modes"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_price(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM1'_items_price", dataset::pison_walmart())?
        .add_target(BenchTarget::Rsonpath(
            "$..bestMarketplacePrice.price",
        ))?
        .add_target(BenchTarget::JsonSki(
            "$.items[*].bestMarketplacePrice.price",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM2'_items_name", dataset::pison_walmart())?
        .add_target(BenchTarget::Rsonpath("$..name"))?
        .add_target(BenchTarget::JsonSki("$.items[*].name"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn wiki_claims_p150(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WP1'_claims_p150", dataset::pison_wiki())?
        .add_target(BenchTarget::Rsonpath(
            "$..P150..mainsnak.property",
        ))?
        .add_target(BenchTarget::JsonSki(
            "$[*].claims.P150[*].mainsnak.property",
        ))?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    paper_parity_benches,
    bestbuy_products_category,
    bestbuy_products_video,
    bestbuy_products_video_only,
    google_map_travel_modes,
    walmart_items_price,
    walmart_items_name,
    wiki_claims_p150
);
