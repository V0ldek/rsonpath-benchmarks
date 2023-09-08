use crate::framework::implementation::Implementation;
use jsonpath_rust::{parser::model::JsonPath, path::json_path_instance, JsonPathValue};
use serde_json::Value;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader},
};
use thiserror::Error;

pub struct JsonpathRust {}

pub struct JsonpathRustResult<'a>(Vec<JsonPathValue<'a, Value>>);

impl Implementation for JsonpathRust {
    type Query = JsonPath;

    type File = Value;

    type Error = JsonpathRustError;

    type Result<'a> = JsonpathRustResult<'a>;

    fn id() -> &'static str {
        "jsonpath-rust"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonpathRust {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;

        Ok(value)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        JsonPath::try_from(query).map_err(JsonpathRustError::JsonPathInstError)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        let inst = json_path_instance(query, file);
        let results = inst.find(file.into());

        Ok(JsonpathRustResult(results))
    }
}

impl<'a> Display for JsonpathRustResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for res in &self.0 {
            writeln!(f, "{}", res.clone().to_data())?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum JsonpathRustError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error("error parsing JSONPath query: '{0}'")]
    JsonPathInstError(<JsonPath as TryFrom<&'static str>>::Error),
}
