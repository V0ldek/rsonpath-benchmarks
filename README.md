# Benchmarks for `rsonpath`

Benchmark suite for [`rsonpath`](https://github.com/v0ldek/rsonpath).

## Dataset

| Bench name            | Path                            | Size      | Depth  | Description |
|-----------------------|---------------------------------|-----------|--------|---|
| `ast`                 | `data/ast`                      | -        | - | JSON representation of the AST of an arbitrary popular C file from Software Heritage. To generate the AST `clang` was used: `clang -Xclang -ast-dump=json -fsyntax-only parse_date.c > ast.json` |
| `crossref`            | `data/crossref`                 | -        | - | Concatenation of the first 100 files from [Crossref](https://www.crossref.org/) [source torrent link](https://academictorrents.com/details/e4287cb7619999709f6e9db5c359dda17e93d515)  |
| `openfood`            | `data/openfood`                 | -        | - | Data extracted from [Open Food Facts API](https://wiki.openfoodfacts.org/Open_Food_Facts_Search_API_Version_2) with `curl "https://world.openfoodfacts.org/cgi/search.pl?action=process&tagtype_0=categories&tag_contains_0=contains&tag_0=cheeses&tagtype_1=labels&&json=1" > /tmp/openfood.json` |
| `twitter`             | `data/twitter`                  | -        | -      | Taken from [`simdjson`](https://github.com/simdjson/simdjson) example benchmarks ([permalink](https://github.com/simdjson/simdjson/blob/960a7ebba149af00628e6a56f9605945f91a15b7/jsonexamples/twitter.json)) |
| `wikidata`            | `data/wikidata`                 | -        | - | Arbitrarily chosen datasets from [Wikidata](https://www.wikidata.org/wiki/Wikidata:Data_access) |

## Prerequisites

1. An appropriate C++ compiler is required for the [`cc` crate](https://lib.rs/crates/cc) to compile the
   JSONSki code.
2. JDK of version at least 8 is required and your `JAVA_HOME` environment variable must be set
   to its location.

On x86_64 Ubuntu the latters can be done by installing `openjdk-17-jdk` and exporting `JAVA_HOME` as
`/usr/lib/jvm/java-1.17.0-openjdk-amd64`.

## Usage

To benchmark a dataset run

```bash
cargo bench --bench <dataset>
```

You can compare the SIMD and no-SIMD versions by disabling the default `simd` feature:

```bash
cargo bench --bench <dataset> --no-default-features
```
