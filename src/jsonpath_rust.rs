use crate::framework::implementation::Implementation;
use jsonpath_rust::{parser::model::JsonPath, path::json_path_instance};
use serde_json::Value;
use std::{
    fs,
    io::{self, BufReader},
};
use thiserror::Error;

pub struct JsonpathRust {}

impl Implementation for JsonpathRust {
    type Query = JsonPath;

    type File = Value;

    type Error = JsonpathRustError;

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

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        let inst = json_path_instance(query, file);
        let results = inst.find(file.into());

        Ok(results.len() as u64)
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
