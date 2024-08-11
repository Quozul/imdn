use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorCode {
    error_code: String,
}

impl ErrorCode {
    pub fn new(error_code: impl ToString) -> Self {
        Self {
            error_code: error_code.to_string(),
        }
    }
}
