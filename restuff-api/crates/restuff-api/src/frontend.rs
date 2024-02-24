use anyhow::Result;
use axum::http::{HeaderMap, Uri};
use axum::response::Response;

use once_cell::sync::Lazy;
use reqwest::redirect;

use crate::{http_error::HttpError, http_utils::response_from_reqwest_body};

static FRONTEND_SERVER: Lazy<String> =
    Lazy::new(|| std::env::var("FRONTEND_HOST").expect("You need to set FRONTEND_HOST"));

#[axum::debug_handler(state=crate::app_state::AppState)]
pub async fn catchall(uri: Uri, headers: HeaderMap) -> Result<Response, HttpError> {
    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .redirect(redirect::Policy::none())
            .build()
            .unwrap()
    });
    let client = &*CLIENT;

    let path = uri.path();
    let path_and_query = uri.path_and_query().map(|paq| paq.as_str()).unwrap_or(path);

    let mut headers = headers
        .into_iter()
        .map(|(key, value)| {
            (
                key.unwrap().to_string().try_into().unwrap(),
                value.to_str().unwrap().try_into().unwrap(),
            )
        })
        .collect::<reqwest::header::HeaderMap>();

    headers.remove("Host");
    headers.remove("Connection");
    headers.remove("Cookies");

    let builder = client
        .get(format!("{}{}", *FRONTEND_SERVER, path_and_query))
        .headers(headers);

    let res = builder.send().await?;

    response_from_reqwest_body(res)
}
