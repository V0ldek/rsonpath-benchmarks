macro_rules! dataset {
    ($e:expr) => {
        concat! {"./data", "/", $e}
    };
}

pub const fn ast() -> &'static str {
    dataset!("ast/ast.json")
}

pub fn crossref(size: u32) -> String {
    format!(dataset!("crossref/crossref{}.json"), size)
}

pub const fn openfood() -> &'static str {
    dataset!("openfood/openfood.json")
}

pub const fn twitter() -> &'static str {
    dataset!("twitter/twitter.json")
}

pub const fn pison_bestbuy_large() -> &'static str {
    dataset!("pison/bestbuy_large_record.json")
}

pub const fn pison_google_map_large() -> &'static str {
    dataset!("pison/google_map_large_record.json")
}

pub const fn pison_nspl_large() -> &'static str {
    dataset!("pison/nspl_large_record.json")
}

pub const fn pison_twitter_large() -> &'static str {
    dataset!("pison/twitter_large_record.json")
}

pub const fn pison_walmart_large() -> &'static str {
    dataset!("pison/walmart_large_record.json")
}

pub const fn pison_wiki_large() -> &'static str {
    dataset!("pison/wiki_large_record.json")
}
