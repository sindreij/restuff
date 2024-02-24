use serde::{de::DeserializeOwned, Deserialize};
use serde_json;

#[derive(Deserialize)]
pub struct SrpcQueryParams {
    input: String,
}

impl SrpcQueryParams {
    pub fn get_input<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.input)
    }
}
