pub trait Implementation<'a>: Sized {
    type Query;
    type File;
    type Error: std::error::Error;

    fn new() -> Result<Self, Self::Error>;

    fn load_file(&'a self, file_path: &str) -> Result<Self::File, Self::Error>;

    fn compile_query(&'a self, query: &str) -> Result<Self::Query, Self::Error>;

    fn run(&'a self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error>;
}

pub struct PreparedQuery<'a, I: Implementation<'a>> {
    pub(crate) implementation: &'a I,
    pub(crate) query: I::Query,
    pub(crate) file: I::File,
}

pub fn prepare<'a, I: Implementation<'a>>(
    implementation: &'a I,
    file_path: &str,
    query: &str,
) -> Result<PreparedQuery<'a, I>, I::Error> {
    let query = implementation.compile_query(query)?;
    let file = implementation.load_file(file_path)?;

    Ok(PreparedQuery {
        implementation,
        query,
        file,
    })
}
