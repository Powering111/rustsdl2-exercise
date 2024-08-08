#[derive(Debug)]
pub enum Error {
    InitFailure(String),
    FileReadFailure,
    JSONParseFailure,
    TextureCreateFailure,
    AlreadyExists,
}
