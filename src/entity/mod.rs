pub mod event;
pub mod user;

fn to_json<T>(value: T) -> String
where
    T: serde::Serialize,
{
    match serde_json::to_string(&value) {
        Ok(json) => json,
        Err(e) => format!("{}:#?", e),
    }
}
