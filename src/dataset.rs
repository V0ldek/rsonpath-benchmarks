macro_rules! dataset {
    ($e:expr) => {
        concat! {"./data", "/", $e}
    };
}

pub const fn ast() -> &'static str {
    dataset!("ast/ast.json")
}

pub const fn crossref() -> &'static str {
    dataset!("crossref/crossref.json")
}

pub const fn openfood() -> &'static str {
    dataset!("openfood/openfood.json")
}

pub const fn twitter() -> &'static str {
    dataset!("twitter/twitter.json")
}