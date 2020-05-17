pub struct Error {
    pub error: String
}

impl Error {
    pub fn error(error: &str) -> Error {
        Error {
            error: error.to_string()
        }
    }
}
