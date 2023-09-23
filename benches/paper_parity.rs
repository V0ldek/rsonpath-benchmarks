use rsonpath_benchmarks::prelude::*;

pub fn twitter_entities_urls(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("TT1_entities_urls", dataset::pison_twitter_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$[*].entities.urls[*].url")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_text(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("TT2_text", dataset::pison_twitter_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$[*].text")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB1_products_category", dataset::pison_bestbuy_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.products[*].categoryPath[*].id")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB2_products_video", dataset::pison_bestbuy_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.products[*].videoChapters[*].chapter")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB3_products_video_only", dataset::pison_bestbuy_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.products[*].videoChapters")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_routes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("GMD1_routes", dataset::pison_google_map_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$[*].routes[*].legs[*].steps[*].distance.text")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn google_map_travel_modes(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("GMD2_travel_modes", dataset::pison_google_map_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$[*].available_travel_modes")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn nspl_meta_columns(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("NSPL1_meta_columns", dataset::pison_nspl())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.meta.view.columns[*].name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn nspl_data(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("NSPL2_data", dataset::pison_nspl())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.data[*][*][*]")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_price(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM1_items_price", dataset::pison_walmart_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.items[*].bestMarketplacePrice.price")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM2_items_name", dataset::pison_walmart_large())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$.items[*].name")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn wiki_claims_p150(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WP1_claims_p150", dataset::pison_wiki())?
        .do_not_measure_file_load_time()
        .add_rsonpath_and_jsonski("$[*].claims.P150[*].mainsnak.property")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    paper_parity_benches,
    twitter_entities_urls,
    twitter_text,
    bestbuy_products_category,
    bestbuy_products_video,
    bestbuy_products_video_only,
    google_map_routes,
    google_map_travel_modes,
    nspl_meta_columns,
    nspl_data,
    walmart_items_price,
    walmart_items_name,
    wiki_claims_p150
);
