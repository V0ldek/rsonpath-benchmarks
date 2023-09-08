use std::fmt::Display;

pub trait Implementation: Sized {
    type Query;
    type File;
    type Error: std::error::Error + Sync + Send + 'static;
    type Result<'a>: Display;

    fn id() -> &'static str;

    fn new() -> Result<Self, Self::Error>;

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error>;

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error>;

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error>;
}

pub struct PreparedQuery<I: Implementation> {
    pub(crate) implementation: I,
    pub(crate) id: &'static str,
    pub(crate) query: I::Query,
    pub(crate) file: File<I::File>,
}

pub(crate) enum File<F> {
    NeedToLoad(String),
    AlreadyLoaded(F),
}

impl<F> File<F> {
    fn from_path(path: &str) -> File<F> {
        File::NeedToLoad(path.to_string())
    }

    fn from_file(file: F) -> File<F> {
        File::AlreadyLoaded(file)
    }
}

pub(crate) fn prepare<I: Implementation>(
    implementation: I,
    file_path: &str,
    query: &str,
    load_ahead_of_time: bool,
) -> Result<PreparedQuery<I>, I::Error> {
    prepare_with_id(implementation, I::id(), file_path, query, load_ahead_of_time)
}

pub(crate) fn prepare_with_id<I: Implementation>(
    implementation: I,
    id: &'static str,
    file_path: &str,
    query: &str,
    load_ahead_of_time: bool,
) -> Result<PreparedQuery<I>, I::Error> {
    let query = implementation.compile_query(query)?;

    let file = if load_ahead_of_time {
        File::from_file(implementation.load_file(file_path)?)
    } else {
        File::from_path(file_path)
    };

    Ok(PreparedQuery {
        implementation,
        id,
        query,
        file,
    })
}
