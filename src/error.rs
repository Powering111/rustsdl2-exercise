#[derive(Debug)]
pub enum Error {
    FileReadFailure,
    JSONParseFailure,
    TextureCreateFailure,
}
