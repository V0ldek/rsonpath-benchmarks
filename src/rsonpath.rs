use ouroboros::self_referencing;
use rsonpath_lib::{
    engine::{recursive::RecursiveEngine, result::CountResult, Compiler, Engine, Input},
    query::JsonPathQuery,
};
use std::fs;
use thiserror::Error;

use crate::framework::implementation::Implementation;

pub struct Rsonpath {}

#[self_referencing()]
pub struct RsonpathQuery {
    query: JsonPathQuery,
    #[borrows(query)]
    #[not_covariant]
    engine: RecursiveEngine<'this>,
}

impl Implementation for Rsonpath {
    type Query = RsonpathQuery;

    type File = Input;

    type Error = RsonpathError;

    fn id() -> &'static str {
        "rsonpath"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Rsonpath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let mut contents = fs::read_to_string(file_path).expect("Reading from file failed.");
        let input = Self::File::new(&mut contents);

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = JsonPathQuery::parse(query).unwrap();

        let rsonpath = RsonpathQuery::try_new(query, |query| {
            RecursiveEngine::compile_query(query).map_err(RsonpathError::CompilerError)
        })?;

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        query
            .with_engine(|engine| engine.run::<CountResult>(file).map(|x| x.get() as u64))
            .map_err(|err| err.into())
    }
}

#[derive(Error, Debug)]
pub enum RsonpathError {
    #[error(transparent)]
    CompilerError(#[from] rsonpath_lib::query::error::CompilerError),
    #[error(transparent)]
    EngineError(#[from] rsonpath_lib::engine::error::EngineError),
    #[error("something happened")]
    Unknown(),
}
