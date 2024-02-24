pub trait SrpcInput {
    fn generate_ts_input_type() -> String;
}

impl SrpcInput for i32 {
    fn generate_ts_input_type() -> String {
        "number".to_string()
    }
}

impl SrpcInput for i64 {
    fn generate_ts_input_type() -> String {
        "number".to_string()
    }
}

impl SrpcInput for String {
    fn generate_ts_input_type() -> String {
        "string".to_string()
    }
}
