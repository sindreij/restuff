pub trait TsInput {
    fn generate_ts_input_type() -> String;
}

impl TsInput for i32 {
    fn generate_ts_input_type() -> String {
        "number".to_string()
    }
}

impl TsInput for i64 {
    fn generate_ts_input_type() -> String {
        "number".to_string()
    }
}

impl TsInput for String {
    fn generate_ts_input_type() -> String {
        "string".to_string()
    }
}
