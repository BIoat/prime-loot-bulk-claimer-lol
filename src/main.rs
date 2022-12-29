use image::*;
use reqwest;

use image::DynamicImage;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{Bytes, Read};
use std::{path::Path, process::Stdio};
use thirtyfour::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::task::spawn_blocking;
async fn ispng(bytes: &[u8]) -> Result<bool, Box<dyn Error>> {
    // Detect the format of the image
    let format = image::guess_format(bytes)?;
    if format == image::ImageFormat::Png {
        Ok(true)
    } else {
        Ok(false)
    }
}
async fn claim(account: Acc) -> WebDriverResult<()> {
    let client = reqwest::Client::new();
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto("https://gaming.amazon.com/loot/lol10").await?;
    let loginb = driver
        .query(By::XPath(
            "/html/body/div[1]/div/div/nav/div/div/div/div/div[2]/div/div[1]/button",
        ))
        .first()
        .await?;

    loginb.wait_until().clickable().await?;
    loginb.click().await?;

    let amazon_e = driver.query(By::Id("ap_email")).first().await?;
    amazon_e.send_keys(&account.username).await?;

    let amazon_p = driver.query(By::Id("ap_password")).first().await?;
    amazon_p.send_keys(&account.password).await?;

    driver
        .query(By::Id("signInSubmit"))
        .first()
        .await?
        .click()
        .await?;
    let amazon_e = driver.query(By::Id("ap_email")).first().await?;
    amazon_e.send_keys(&account.username).await?;

    let amazon_p = driver.query(By::Id("ap_password")).first().await?;
    amazon_p.send_keys(account.password).await?;

    let captcha_pic = driver
        .query(By::Id("auth-captcha-image-container"))
        .first()
        .await?;
    
    captcha_pic.wait_until().displayed().await?;

    let inner_html = captcha_pic.inner_html().await?.replace("&amp;", "&");
    let re = Regex::new(r#"src="(.+?)""#).unwrap();

    let src = re.captures(&inner_html).unwrap().get(1).unwrap().as_str();
    let mut response = client.get(src).send().unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();

    let mut bytes = buffer.clone();
    let mut clone = bytes.clone();
    let is_png = spawn_blocking(move || match image::guess_format(&buffer) {
        Ok(image::ImageFormat::Png) => true,
        _ => false,
    })
    .await
    .unwrap();
    if is_png {
        println!("convert to GIF: {}", is_png);

        let image =
            image::load_from_memory_with_format(&mut *bytes, image::ImageFormat::Png).unwrap();
        let frame = gif::Frame::from_rgba(200, 70, &mut *image.to_rgba8());
        let mut encoder = gif::Encoder::new(&mut *clone, frame.width, frame.height, &[]).unwrap();
        encoder.write_frame(&frame).unwrap();
    }
    let mut cmd = Command::new("../out/captchasolver");

    // to stdin it can now be used as an asynchronous writer.
    cmd.stdout(Stdio::piped());
    cmd.stdin(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to spawn command");

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");
    stdin
        .write_all(&bytes)
        .await
        .expect("could not write to stdin");

    drop(stdin);

    let op = child.wait_with_output().await?;
    driver.quit().await?;

    Ok(())
}
enum AccountType {
    Amazon,
    Riot,
}
struct Acc {
    accounttype: AccountType,
    username: String,
    password: String,
    claimed: bool,
}

#[tokio::main]
async fn main() {
    let account = Acc {
        accounttype: AccountType::Amazon,
        username: "test".to_string(),
        password: "password".to_string(),
        claimed: false,
    };

    claim(account).await;
}
