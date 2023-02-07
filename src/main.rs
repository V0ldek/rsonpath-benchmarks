use rsonpath_benchmarks::framework::implementation::Implementation;
use rsonpath_benchmarks::rust_jsonski::JsonSki;

fn main() {
    let jsonski = JsonSki::new().unwrap();
    let query = jsonski.compile_query("$[*].entities.urls[*].url").unwrap();
    let file = jsonski
        .load_file(
            "/home/v0ldek/rsonpath/crates/rsonpath-benchmarks/data/pison/twitter_large_record.json",
        )
        .unwrap();

    for _ in 0..100 {
        let result = jsonski.run(&query, &file).unwrap();
        println!("{result}");
    }
}
