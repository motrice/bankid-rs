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
use image::png::PngEncoder;
use std::io::Write;

pub mod domain;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub struct BankIdClient {
    end_point: String,
}

impl BankIdClient {
    pub fn new(end_point: &str) -> BankIdClient {
        BankIdClient { end_point: String::from(end_point)}
    }


    pub async fn qr_code_png(&self, uri: &str) -> Option<String> {
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

                match PngEncoder::new(png_buffer.by_ref())
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

    pub async fn auth(&self, client: reqwest::Client, personal_number: Option<&str>, end_user_ip: &str) -> Result<domain::AuthSignResponse> {
        let auth_req = domain::AuthRequestData {
            personal_number: match personal_number {
                Some(s) => Some(String::from(s)),
                None => None
            },
            end_user_ip: String::from(end_user_ip),
            requirement: None
        };

        let auth_res = client
            .post(&format!("{}/{}", &self.end_point, "auth"))
            .json(&auth_req)
            .send()
            .await?
            .json().await?;

        Ok(auth_res)
    }

    pub async fn sign(&self, client: reqwest::Client, personal_number: Option<String>, end_user_ip: &str, user_visible_data: &str, user_non_visible_data: Option<String>) -> Result<domain::AuthSignResponse> {

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
            .post(&format!("{}/{}", &self.end_point, "sign"))
            .json(&sign_req)
            .send()
            .await?
            .json().await?;

        Ok(auth_res)
    }


    pub async fn collect(&self, client: reqwest::Client, order_ref: &str) -> Result<domain::CollectResponse> {
        let collect_req = domain::CollectRequestData {
            order_ref: String::from(order_ref)
        };

        let auth_res = client
            .post(&format!("{}/{}", &self.end_point, "collect"))
            .json(&collect_req)
            .send()
            .await?
            .json().await?;

        Ok(auth_res)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
