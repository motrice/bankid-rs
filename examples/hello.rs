type GenericError = Box<dyn std::error::Error + Send + Sync>;
// type Result<T> = std::result::Result<T, GenericError>;
#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let client = bankid_rs::BankIdClient::new(
        "https://appapi2.test.bankid.com/rp/v5", 
        "cert/test/bankid.crt", 
        "cert/test/FPTestcert3_20200618.p12", 
        "qwerty123", 
        10)?;
    let auth_res = client.auth(Some("191212121212"), "127.0.0.1").await?;
    println!("Response order_ref: {}", auth_res.order_ref);

    let collect_response =  client.collect(&auth_res.order_ref).await?;

    println!("Collect Response: {}", collect_response);

    Ok(())
}