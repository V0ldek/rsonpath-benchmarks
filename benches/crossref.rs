use rsonpath_benchmarks::prelude::*;

pub fn doi(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("DOI", dataset::crossref())?
        .add_all_targets_except_jsonski("$..DOI")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn title(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("title", dataset::crossref())?
        .add_all_targets_except_jsonski("$..title")?
        .add_target(BenchTarget::JsonSki("$[*].message.items[*].title"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn orcid(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("orcid", dataset::crossref())?
        .add_all_targets_except_jsonski("$..author..ORCID")?
        .add_target(BenchTarget::JsonSki("$[*].message.items[*].author[*].ORCID"))?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn affiliation(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("affiliation", dataset::crossref())?
        .add_all_targets_except_jsonski("$..affiliation..name")?
        .add_target(BenchTarget::JsonSki("$[*].message.items[*].author[*].affiliation[*].name"))?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(crossref_benches, affiliation, orcid, title, doi);
