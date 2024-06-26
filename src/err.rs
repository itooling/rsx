#[derive(Debug)]
pub enum BaseErr {
    StringError(String),
    EcdhError(String),
}
