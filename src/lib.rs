extern crate reqwest;
extern crate serde;
extern crate qrcode;
extern crate image;
extern crate base64;
extern crate uuid;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use qrcode::QrCode;
use image::Luma;
use image::png::PNGEncoder;
use std::io::Write;

use tokio::timer::delay;
use std::time::Duration;

pub mod domain;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub async fn qr_code_png(uri: &str) -> Option<String> {
    // Encode some data into bits.
    return match QrCode::new(&uri) {
        Ok(code) => {
            // Render the bits into an image.
            let qr_image = code.render::<Luma<u8>>().build();

            let width = qr_image.width();
            let height = qr_image.height();
            let color_type = <image::Luma<u8> as image::Pixel>::COLOR_TYPE; // . image::ColorType::Gray(u8::max_value());

            let mut png_buffer = Vec::new();
            //let s : String = qr_image;

            match PNGEncoder::new(png_buffer.by_ref())
                .encode(
                    &qr_image,
                    width,
                    height,
                    color_type
                    ) {
                        Ok(_) => info!("Created PNG with QRCode for {}", uri),
                        Err(err) => error!("Failed to create PNG with QRCode for {}", err)
                    };

            Some(base64::encode(&png_buffer))
        }
        Err(err) => {
            error!("Failed to create QRCode for {}: {}", uri, err);
            None
        }
    }
}

pub async fn auth(client: reqwest::Client, end_point: &str, personal_number: Option<&str>, end_user_ip: &str) -> Result<domain::AuthSignResponse> {
    let auth_req = domain::AuthRequestData {
        personal_number: match personal_number {
            Some(s) => Some(String::from(s)),
            None => None
        },
        end_user_ip: String::from(end_user_ip),
        requirement: None
    };

    let auth_res = client
        .post("https://appapi2.test.bankid.com/rp/v5/auth")
        .json(&auth_req)
        .send()
        .await?
        .json().await?;

    Ok(auth_res)
}

pub async fn sign(client: reqwest::Client, end_point: &str, personal_number: Option<String>, end_user_ip: &str, user_visible_data: &str, user_non_visible_data: Option<String>) -> Result<domain::AuthSignResponse> {

    let non_visible = match user_non_visible_data {
        Some(s) => Some(base64::encode(&s)),
        None => None
    };
    let sign_req = domain::SignRequestData {
        personal_number: match personal_number {
            Some(s) => Some(String::from(s)),
            None => None
        },
        end_user_ip: String::from(end_user_ip),
        requirement: None,
        user_visible_data: base64::encode(&user_visible_data),
        user_non_visible_data: non_visible
    };

    info!("sign req {}", sign_req.user_visible_data);
    let auth_res = client
        .post("https://appapi2.test.bankid.com/rp/v5/sign")
        .json(&sign_req)
        .send()
        .await?
        .json().await?;

    Ok(auth_res)
}


pub async fn collect(client: reqwest::Client, end_point: &str, order_ref: &str) -> Result<domain::CollectResponse> {
    let collect_req = domain::CollectRequestData {
        order_ref: String::from(order_ref)
    };
    
    let auth_res = client
        .post("https://appapi2.test.bankid.com/rp/v5/collect")
        .json(&collect_req)
        .send()
        .await?
        .json().await?;

    Ok(auth_res)
}

pub async fn poll_collect_until_completed(client: reqwest::Client, end_point: &str, order_ref: &str) -> Result<domain::CollectResponse> { //Result<String> {
    loop {
        let when = tokio::clock::now() + Duration::from_millis(2000);
        let poll_wait = delay(when).await;
        let collected  = collect(client.clone(), end_point, order_ref).await;
        let req_res = (collected, poll_wait);

        match req_res.0 {
            Ok(value) => {
                info!("BankID status for order {} is {}", value.order_ref, value.status);
                match value.status {
                    domain::Status::Complete => {
                        break Ok(value)
                    },
                    domain::Status::Pending => {
                    },
                    domain::Status::Failed => {
                        break Ok(value)
                    }
                }
            },
            Err(err) => {
                error!("Error: {}", err);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
