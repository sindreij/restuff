use axum::Json;

pub trait SrpcOutput {
    fn generate_zod_schema() -> String;
}

impl SrpcOutput for () {
    fn generate_zod_schema() -> String {
        "z.null()".to_string()
    }
}

impl SrpcOutput for i32 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl SrpcOutput for i64 {
    fn generate_zod_schema() -> String {
        "z.number()".to_string()
    }
}

impl SrpcOutput for chrono::DateTime<chrono::Utc> {
    fn generate_zod_schema() -> String {
        "z.string().datetime()".to_string()
    }
}

impl SrpcOutput for String {
    fn generate_zod_schema() -> String {
        "z.string()".to_string()
    }
}

impl<T, E> SrpcOutput for Result<T, E>
where
    T: SrpcOutput,
{
    // This only works when E does not return 200. So should maybe look into that.. or not. I don't know
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> SrpcOutput for Option<T>
where
    T: SrpcOutput,
{
    fn generate_zod_schema() -> String {
        format!("z.nullable({})", T::generate_zod_schema())
    }
}

impl<T> SrpcOutput for Json<T>
where
    T: SrpcOutput,
{
    fn generate_zod_schema() -> String {
        T::generate_zod_schema()
    }
}

impl<T> SrpcOutput for Vec<T>
where
    T: SrpcOutput,
{
    fn generate_zod_schema() -> String {
        format!("z.array({})", T::generate_zod_schema())
    }
}
