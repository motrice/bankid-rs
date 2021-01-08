type GenericError = Box<dyn std::error::Error + Send + Sync>;
// type Result<T> = std::result::Result<T, GenericError>;
#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let client = bankid_rs::BankIdClient::new_from_env()?;
    let auth_res = client.auth(Some("191212121212"), "127.0.0.1").await?;
    println!("Response order_ref: {}", auth_res.order_ref);

    let collect_response =  client.collect(&auth_res.order_ref).await?;

    println!("Collect Response: {}", collect_response);

    Ok(())
}