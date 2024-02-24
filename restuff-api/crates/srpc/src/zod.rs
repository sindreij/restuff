use axum::Json;

pub trait ZodGen {
    fn generate_zod_schema() -> String;
}

impl ZodGen for () {
    fn generate_zod_schema() -> String {
        "z.null()".to_string()
    }
}

impl ZodGen for i32 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl ZodGen for i64 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl ZodGen for chrono::DateTime<chrono::Utc> {
    fn generate_zod_schema() -> String {
        "z.string().datetime()".to_string()
    }
}

impl ZodGen for String {
    fn generate_zod_schema() -> String {
        "z.string()".to_string()
    }
}

impl<T, E> ZodGen for Result<T, E>
where
    T: ZodGen,
{
    // This only works when E does not return 200. So should maybe look into that.. or not. I don't know
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> ZodGen for Option<T>
where
    T: ZodGen,
{
    fn generate_zod_schema() -> String {
        format!("z.nullable({})", T::generate_zod_schema())
    }
}

impl<T> ZodGen for Json<T>
where
    T: ZodGen,
{
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> ZodGen for Vec<T>
where
    T: ZodGen,
{
    fn generate_zod_schema() -> String {
        format!("z.array({})", T::generate_zod_schema())
    }
}
