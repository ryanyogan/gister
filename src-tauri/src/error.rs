use std::fmt::Display;

use reqwest::StatusCode;

#[derive(Debug)]
pub struct TauriError {
    pub message: &'static str,
}

impl From<reqwest::Error> for TauriError {
    fn from(value: reqwest::Error) -> Self {
        let error_message = match value.status().unwrap() {
            StatusCode::FORBIDDEN => "This endpoint requires a token",
            StatusCode::BAD_REQUEST => "There was a problem with your request",
            _ => "Something went wrong handling this request",
        };

        TauriError {
            message: error_message,
        }
    }
}

impl serde::Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Display for TauriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
