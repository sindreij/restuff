use axum::{body::Body, http::HeaderName, response::Response};

use crate::http_error::HttpError;

pub fn response_from_reqwest_body(
    res: reqwest::Response,
    // cache: Cache,
) -> Result<Response, HttpError> {
    let mut response_builder = Response::builder()
        .status(res.status().as_u16())
        .header("Cache-Control", "no-cache");

    let headers = response_builder
        .headers_mut()
        .ok_or(HttpError::bad_request("Headers not found"))?;

    for (key, value) in res.headers() {
        if key != "connection" && key != "cache-control" {
            headers.append(
                HeaderName::try_from(key.to_string()).unwrap(),
                value.to_str().unwrap().parse().unwrap(),
            );
        }
    }

    // let cache_header = match cache {
    //     Cache::KeepUpstream => res
    //         .headers()
    //         .get("cache-control")
    //         .cloned()
    //         .unwrap_or_else(|| "".parse().unwrap()),
    //     Cache::No => "no-cache".parse().unwrap(),
    // };

    // headers.append(
    //     "cache-control",
    //     cache_header.to_str().unwrap().parse().unwrap(),
    // );

    Ok(response_builder.body(Body::from_stream(res.bytes_stream()))?)
}

// #[derive(Debug, Clone, Copy)]
// pub enum Cache {
//     KeepUpstream,
//     No,
// }
