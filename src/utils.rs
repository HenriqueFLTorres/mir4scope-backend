use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[allow(unused)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn get_response<T>(
    client: &ClientWithMiddleware,
    request_url: String,
) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let response = client
        .get(request_url)
        .header("User-Agent", APP_USER_AGENT)
        .send()
        .await
        .unwrap();

    let body = response.text().await?;
    let result = serde_json::from_str(&body)?;
    Ok(result)
}

pub fn nft_description_error(message: &str, nft_data: serde_json::Value) -> std::string::String {
    return format!(
        "\n {}: \n transport_id: {}, seq: {}, character_name: {} \n",
        message, nft_data["transport_id"], nft_data["seq"], nft_data["character_name"]
    );
}

pub struct AppState {
    pub db: Pool<Postgres>,
    pub client: ClientWithMiddleware,
}
