use axum::{http::StatusCode, response::IntoResponse};

pub struct HttpError {
    code: StatusCode,
    message: String,
    inner: Option<anyhow::Error>,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        // eprintln!("[{}] {}", self.code, self.message);

        if let Some(inner) = &self.inner {
            eprintln!("{:?}", inner);
        }

        (self.code, self.message).into_response()
    }
}

#[allow(unused)]
impl HttpError {
    pub fn not_found(message: &str) -> Self {
        HttpError {
            code: StatusCode::NOT_FOUND,
            message: message.into(),
            inner: None,
        }
    }

    pub fn bad_request<M: Into<String>>(msg: M) -> Self {
        HttpError {
            code: StatusCode::BAD_REQUEST,
            message: msg.into(),
            inner: None,
        }
    }

    pub fn not_found_with_cause<E: Into<anyhow::Error>>(inner: E) -> Self {
        HttpError {
            code: StatusCode::NOT_FOUND,
            message: "404 - not found".into(),
            inner: Some(inner.into()),
        }
    }

    pub fn unauthorized(msg: &str) -> Self {
        HttpError {
            code: StatusCode::UNAUTHORIZED,
            message: msg.into(),
            inner: None,
        }
    }
}

impl<T> From<T> for HttpError
where
    T: Into<anyhow::Error>,
{
    fn from(error: T) -> Self {
        HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "500 - internal server error".into(),
            inner: Some(error.into()),
        }
    }
}
