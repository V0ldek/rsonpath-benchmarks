use rsonpath_benchmarks::prelude::*;

pub fn twitter_entities_urls(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("TT1_entities_urls", dataset::pison_twitter())?
        .add_target(BenchTarget::JSurfer("$[*].entities.urls[*].url"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn twitter_text(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("TT2_text", dataset::pison_twitter())?
        .add_target(BenchTarget::JSurfer("$[*].text"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_category(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB1_products_category", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::JSurfer("$.products[*].categoryPath[*].id"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB2_products_video", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::JSurfer("$.products[*].videoChapters[*].chapter"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn bestbuy_products_video_only(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("BB3_products_video_only", dataset::pison_bestbuy_large())?
        .add_target(BenchTarget::JSurfer("$.products[*].videoChapters"))?
        .finish();

    benchset.run(c);

    Ok(())
}



pub fn walmart_items_price(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM1_items_price", dataset::pison_walmart())?
        .add_target(BenchTarget::JSurfer("$.items[*].bestMarketplacePrice.price"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn walmart_items_name(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("WM2_items_name", dataset::pison_walmart())?
        .add_target(BenchTarget::JSurfer("$.items[*].name"))?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    paper_parity_jsurfer_benches,
    twitter_entities_urls,
    twitter_text,
    bestbuy_products_category,
    bestbuy_products_video,
    bestbuy_products_video_only,
    walmart_items_price,
    walmart_items_name,
);
