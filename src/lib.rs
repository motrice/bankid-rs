extern crate reqwest;
extern crate serde;
extern crate qrcode;
extern crate image;
extern crate base64;
extern crate uuid;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use std::fs::File;
use std::io::Read;
use std::env;

use qrcode::QrCode;
use image::Luma;
use image::png::PngEncoder;
use std::io::Write;
use std::time::Duration;

pub mod domain;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

const DEFAULT_TIMEOUT: u64 = 10;
pub struct BankIdClient {
    end_point: String,
    client: reqwest::Client
}

impl BankIdClient {
    pub fn new(end_point: &str, server_cert_filename: &str,
        client_cert_filename: &str,
        client_cert_password: &str, client_timeout_secs: u64) -> Result<BankIdClient> {
        // read server certificate
        let mut buf = Vec::new();
        File::open(server_cert_filename)?.read_to_end(&mut buf)?;
        let cert = reqwest::Certificate::from_pem(&buf)?;

        // read client certificate
        let mut buf = Vec::new();
        File::open(client_cert_filename)?
            .read_to_end(&mut buf)?;
        let pkcs12 = reqwest::Identity::from_pkcs12_der(&buf, client_cert_password)?;

        let client = reqwest::Client::builder()
            .identity(pkcs12)
            .add_root_certificate(cert)
            .gzip(true)
            .timeout(Duration::from_secs(client_timeout_secs))
            .build()?;

        Ok(BankIdClient { end_point: String::from(end_point), client: client})
    }

    /**
     * Does not yet work
     */
    pub fn new_from_env() -> Result<BankIdClient> {
        let bankid_url = env::var("BANKID_URL").expect("Missing required BANKID_URL environment variable");
        let server_cert = env::var("BANKID_SERVER_CERT").expect("Missing required BANKID_SERVER_CERT environment variable");
        let client_cert = env::var("BANKID_CLIENT_CERT").expect("Missing required BANKID_CLIENT_CERT environment variable");
        let client_timeout_secs = match env::var("BANKID_CLIENT_TIMEOUT_SECS") {
            Ok(value) => match value.parse::<u64>() {
                Ok(timeout) => timeout,
                Err(err) => {
                    warn!("Value of CLIENT_TIMEOUT_SECS environment variable is invalid {}. Using default timeout {}", err, DEFAULT_TIMEOUT);
                    DEFAULT_TIMEOUT
                }
            },
            Err(err) => {
                warn!("Missing CLIENT_TIMEOUT_SECS environment variable {}, using default timeout {}", err, DEFAULT_TIMEOUT);
                DEFAULT_TIMEOUT
            }
        };
        let cert = reqwest::Certificate::from_pem(&server_cert.as_bytes())?;
        println!("1");
        let pkcs12 = reqwest::Identity::from_pem(&client_cert.as_bytes())?;
        println!("2");
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .identity(pkcs12)
            .add_root_certificate(cert)
            .gzip(true)
            .timeout(Duration::from_secs(client_timeout_secs))
            .build()?;

        Ok(BankIdClient { end_point: bankid_url, client: client})
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

    pub async fn auth(&self, personal_number: Option<&str>, end_user_ip: &str) -> Result<domain::AuthSignResponse> {
        let auth_req = domain::AuthRequestData {
            personal_number: match personal_number {
                Some(s) => Some(String::from(s)),
                None => None
            },
            end_user_ip: String::from(end_user_ip),
            requirement: None
        };

        let auth_res = self.client
            .post(&format!("{}/{}", &self.end_point, "auth"))
            .json(&auth_req)
            .send()
            .await?
            .json().await?;

        Ok(auth_res)
    }

    pub async fn sign(&self, personal_number: Option<String>, end_user_ip: &str, user_visible_data: &str, user_non_visible_data: Option<String>) -> Result<domain::AuthSignResponse> {

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
        let auth_res = self.client
            .post(&format!("{}/{}", &self.end_point, "sign"))
            .json(&sign_req)
            .send()
            .await?
            .json().await?;

        Ok(auth_res)
    }


    pub async fn collect(&self, order_ref: &str) -> Result<domain::CollectResponse> {
        let collect_req = domain::CollectRequestData {
            order_ref: String::from(order_ref)
        };

        let auth_res = self.client
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
