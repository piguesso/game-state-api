pub struct Error {
    pub message: String,
    pub status_code: i32,
}

impl Error {
    pub fn new(message: String, status_code: i32) -> Error {
        Error {
            message,
            status_code,
        }
    }
}
