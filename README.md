# Benchmarks for `rsonpath`

Benchmark suite for [`rsonpath`](https://github.com/v0ldek/rsonpath).


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

## Download the dataset

On linux system with `wget` installed run the script `sh dl.sh`. Otherwise you can download manually the dataset and put them in the correct folder.

For more information, refers to:

* Crossref: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7225594.svg)](https://doi.org/10.5281/zenodo.7225594)
* Twitter: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7225577.svg)](https://doi.org/10.5281/zenodo.7225577)
* AST: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7225575.svg)](https://doi.org/10.5281/zenodo.7225575)

For the benchmark to work, the directory layout should be as follows:

```
── data
   ├── ast
   │   └── ast.json
   ├── crossref
   │   ├── crossref0.json
   │   ├── crossref16.json
   │   ├── crossref1.json
   │   ├── crossref2.json
   │   ├── crossref4.json
   │   └── crossref8.json
   └── twitter
       └── twitter.json
```

The md5sum of all the json file:

* `./data/ast/ast.json` 1fa4d1cccd576d3b8c41ae2b3e41ea9c 
* `./data/twitter/twitter.json` 7d3c5866a899ab6c1afb010bc31f821d 
* `./data/crossref/crossref8.json` 65fe2be99ae61662c90dcfd160f1118b 
* `./data/crossref/crossref16.json` dd0e20c8a420428ace481fa058954936 
* `./data/crossref/crossref2.json` f1a859af978b668e2e2bee4bfe7c53fe 
* `./data/crossref/crossref0.json` c6eb37d4a7eb25c05dd3e32bc7dab3db 
* `./data/crossref/crossref1.json` 3b5e505634c36158d1ae0027f7f67d83 
* `./data/crossref/crossref4.json` 2ec1caa9a6be75e5ba8439ebfebda22d 

## Usage

To benchmark a dataset run

```bash
cargo bench --bench <dataset>
```

You can compare the SIMD and no-SIMD versions by disabling the default `simd` feature:

```bash
cargo bench --bench <dataset> --no-default-features
```

The folder `target/criterion` contains all the information needed to plot the experiment.

## Plotting

To plot the result once the bench done.

```bash
python3 charts/charts.py
```

You can also provide a path to a `criterion` folder:

```bash
python3 charts/charts.py exps/chetemi
```

The plot will be saved in the `plot.png` file of the current directory. (I know, it could be better).

## Statistics

Two statistics scripts are available:

* One about the dataset: 

```python
python3 charts/dataset_stat.py
```

It will plot on stdout some informations about each json-file in the `data` folder. Be aware that it will
load the file in memory, in python. Expect it to be slow and memory consumming.

* About the queries

```python
python3 charts/queries_stat.py
```

This script will assume you run the benchmark (and look into the directory `target/criterion`) to extract the list
of queries. It will then compute some parameters and the number of match with `rsonpath`. The binary of `rsonpath`
should be in the path (run `cargo install rsonpath`).

