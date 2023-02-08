use rsonpath_benchmarks::prelude::*;

pub fn vitamins_tags(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("vitamins_tags", dataset::openfood())?
        .add_all_targets("$.products[*].vitamins_tags")?
        .finish();

    benchset.run(c);

    Ok(())
}
pub fn vitamins_tags_descendant(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("vitamins_tags_descendant", dataset::openfood())?
        .add_all_targets_except_jsonski("$..vitamins_tags")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn added_countries_tags(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("added_counties_tags", dataset::openfood())?
        .add_all_targets("$.products[*].added_countries_tags")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn added_countries_tags_descendant(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("added_countries_tags_descendant", dataset::openfood())?
        .add_all_targets_except_jsonski("$..added_countries_tags")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn specific_ingredients(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("specific_ingredients", dataset::openfood())?
        .add_all_targets("$.products[*].specific_ingredients[*].ingredient")?
        .finish();

    benchset.run(c);

    Ok(())
}

pub fn specific_ingredients_descendant(c: &mut Criterion) -> Result<(), BenchmarkError> {
    let benchset = Benchset::new("specific_ingredients_descendant", dataset::openfood())?
        .add_all_targets_except_jsonski("$..specific_ingredients..ingredient")?
        .finish();

    benchset.run(c);

    Ok(())
}

benchsets!(
    openfood_benches,
    specific_ingredients,
    specific_ingredients_descendant,
    added_countries_tags,
    added_countries_tags_descendant,
    vitamins_tags,
    vitamins_tags_descendant,
);
