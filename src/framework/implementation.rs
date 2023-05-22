pub trait Implementation: Sized {
    type Query;
    type File;
    type Error: std::error::Error + Sync + Send + 'static;

    fn id() -> &'static str;

    fn new() -> Result<Self, Self::Error>;

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error>;

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error>;

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error>;
}

pub struct PreparedQuery<I: Implementation> {
    pub(crate) implementation: I,
    pub(crate) id: &'static str,
    pub(crate) query: I::Query,
    pub(crate) file_path: String,
}

pub(crate) fn prepare<I: Implementation>(
    implementation: I,
    file_path: &str,
    query: &str,
) -> Result<PreparedQuery<I>, I::Error> {
    prepare_with_id(implementation, I::id(), file_path, query)
}

pub(crate) fn prepare_with_id<I: Implementation>(
    implementation: I,
    id: &'static str,
    file_path: &str,
    query: &str,
) -> Result<PreparedQuery<I>, I::Error> {
    let query = implementation.compile_query(query)?;

    Ok(PreparedQuery {
        implementation,
        id,
        query,
        file_path: file_path.to_owned(),
    })
}
