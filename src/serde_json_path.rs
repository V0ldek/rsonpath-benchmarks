use crate::framework::implementation::Implementation;
use serde_json::Value;
use serde_json_path::{JsonPath, ParseError};
use std::{
    fs,
    io::{self, BufReader},
};
use thiserror::Error;

pub struct SerdeJsonPath {}

impl Implementation for SerdeJsonPath {
    type Query = JsonPath;

    type File = Value;

    type Error = SerdeJsonPathError;

    fn id() -> &'static str {
        "serde_json_path"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(SerdeJsonPath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;

        Ok(value)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        JsonPath::parse(query).map_err(SerdeJsonPathError::JsonPathParseError)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        Ok(query.query(file).len() as u64)
    }
}

#[derive(Error, Debug)]
pub enum SerdeJsonPathError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonPathParseError(#[from] ParseError),
}
