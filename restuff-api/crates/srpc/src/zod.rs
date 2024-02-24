use axum::Json;

pub trait ZodSchema {
    fn generate_zod_schema() -> String;
}

impl ZodSchema for () {
    fn generate_zod_schema() -> String {
        "z.null()".to_string()
    }
}

impl ZodSchema for i32 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl ZodSchema for i64 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl ZodSchema for chrono::DateTime<chrono::Utc> {
    fn generate_zod_schema() -> String {
        "z.string().datetime()".to_string()
    }
}

impl ZodSchema for String {
    fn generate_zod_schema() -> String {
        "z.string()".to_string()
    }
}

impl<T, E> ZodSchema for Result<T, E>
where
    T: ZodSchema,
{
    // This only works when E does not return 200. So should maybe look into that.. or not. I don't know
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> ZodSchema for Option<T>
where
    T: ZodSchema,
{
    fn generate_zod_schema() -> String {
        format!("z.nullable({})", T::generate_zod_schema())
    }
}

impl<T> ZodSchema for Json<T>
where
    T: ZodSchema,
{
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> ZodSchema for Vec<T>
where
    T: ZodSchema,
{
    fn generate_zod_schema() -> String {
        format!("z.array({})", T::generate_zod_schema())
    }
}
