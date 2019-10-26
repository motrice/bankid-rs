extern crate reqwest;
extern crate serde;
extern crate qrcode;
extern crate image;
extern crate base64;
extern crate uuid;

use qrcode::QrCode;
use image::Luma;
use image::png::PNGEncoder;
use std::io::Write;

use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;

use reqwest::header;
use std::fs::File;
use std::io::Read;

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

            println!("width {} height {}", width, height);

            let mut png_buffer = Vec::new();
            //let s : String = qr_image;

            match PNGEncoder::new(png_buffer.by_ref())
                .encode(
                    &qr_image,
                    width,
                    height,
                    color_type
                    ) {
                        Ok(_) => println!("====> OK"),
                        Err(err) => println!("====> ERROR {}", err)
                    };
            println!("generated image with width {} height {}", width, height);

            Some(base64::encode(&png_buffer))
        }
        Err(err) => {
            println!("ERROR {}", err);
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

pub async fn poll_collect_until_completed(client: reqwest::Client, end_point: &str, order_ref: &str) -> Result<String> {
    println!("poll until completed");
    let mut count : u32 = 0;
    let mut completed = false;
    while !completed {
        let when = tokio::clock::now() + Duration::from_millis(2000);

        let poll_wait = delay(when).await;
        let collect = collect(client.clone(), end_point, order_ref).await;

        let req_res = (collect, poll_wait);

        completed = match req_res.0 {
            Ok(value) => {
                println!("collecto {} {}", value.order_ref, value.status);
                match value.status {
                    domain::Status::Complete => {
                        println!("Status is 'complete']'");
                        true
                    },
                    domain::Status::Pending => {
                        println!("Status is 'pending']'");
                        false
                    },
                    domain::Status::Failed => {
                        println!("Status is 'failed']'");
                        true
                    }
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                false
            }
        };

        if completed {
            println!("Klart!");
        }

        if count>300 {
            println!("Nä nu ger jag upp!");
            completed = true;
        }

        count = count + 1;
    }
    Ok(String::from("så"))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
