pub trait ZodGen {
    fn generate_zod_schema() -> String;
}

impl ZodGen for i32 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl ZodGen for String {
    fn generate_zod_schema() -> String {
        "z.string()".to_string()
    }
}
