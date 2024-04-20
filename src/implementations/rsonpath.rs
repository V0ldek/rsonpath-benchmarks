use crate::framework::implementation::Implementation;
use rsonpath::{
    engine::main::MainEngine,
    input::OwnedBytes,
    result::{Match, Sink},
};
use rsonpath::{
    engine::{Compiler, Engine},
    input::MmapInput,
};
use std::{convert::Infallible, fmt::Display, fs, io};
use thiserror::Error;

pub struct Rsonpath {}
pub struct RsonpathCount {}
pub struct RsonpathMmap {}
pub struct RsonpathMmapCount {}

pub struct RsonpathQuery {
    engine: MainEngine,
}

impl Implementation for Rsonpath {
    type Query = RsonpathQuery;

    type File = OwnedBytes<Vec<u8>>;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Rsonpath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query)?;
        let engine = MainEngine::compile_query(&query)?;

        let rsonpath = RsonpathQuery { engine };

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.engine.matches(file, &mut VoidSink)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathCount {
    type Query = RsonpathQuery;

    type File = OwnedBytes<Vec<u8>>;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_count"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathCount {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query)?;
        let engine = MainEngine::compile_query(&query)?;

        let rsonpath = RsonpathQuery { engine };

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.engine.count(file)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmap {
    type Query = RsonpathQuery;

    type File = MmapInput;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_mmap"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathMmap {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query)?;
        let engine = MainEngine::compile_query(&query)?;

        let rsonpath = RsonpathQuery { engine };

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.engine.matches(file, &mut VoidSink)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmapCount {
    type Query = RsonpathQuery;

    type File = MmapInput;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_mmap_count"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathMmapCount {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query)?;
        let engine = MainEngine::compile_query(&query)?;

        let rsonpath = RsonpathQuery { engine };

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.engine.count(file)?;

        Ok("[not collected]")
    }
}

#[derive(Error, Debug)]
pub enum RsonpathError {
    #[error(transparent)]
    ParseError(#[from] rsonpath_syntax::error::ParseError),
    #[error(transparent)]
    CompilerError(#[from] rsonpath::automaton::error::CompilerError),
    #[error(transparent)]
    EngineError(#[from] rsonpath::engine::error::EngineError),
    #[error(transparent)]
    InputError(#[from] rsonpath::input::error::InputError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("something happened")]
    Unknown(),
}

pub struct MatchDisplay(Vec<Match>);

impl Display for MatchDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.0 {
            writeln!(f, "{m}")?
        }

        Ok(())
    }
}

struct VoidSink;

impl<D> Sink<D> for VoidSink {
    type Error = Infallible;

    fn add_match(&mut self, _data: D) -> Result<(), Self::Error> {
        Ok(())
    }
}
