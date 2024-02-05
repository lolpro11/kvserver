#[tarpc::service]
pub trait KV {
    /// Returns a greeting for name.
    async fn put(key: String, value: String) -> String;
    async fn append(key: String, value: String) -> String;
    async fn get(key: String) -> String;
}