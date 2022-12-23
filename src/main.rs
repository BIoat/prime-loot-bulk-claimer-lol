
use image::{open};

use std::{
    path::{Path},
    time::Duration,
};

use show_image::{create_window, event, ImageInfo, ImageView};
use thirtyfour::prelude::*;
use tokio::time::{sleep};

async fn solve_captcha(buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let image = ImageView::new(ImageInfo::rgba8(200, 70), buffer);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
    Ok(())
}
async fn claim(account: Acc) -> WebDriverResult<()> {
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
    // let innerhtml = captcha_pic.inner_html().await?;
    // println!("inner html: {}", &innerhtml);
    // let fragment = Html::parse_fragment(&innerhtml);
    // let selector = Selector::parse("src").unwrap();
    // for element in fragment.select(&selector) {
    //     println!("inner html: {}", &element.value().name());
    // }

    sleep(Duration::from_millis(1200)).await;
    captcha_pic.screenshot(Path::new("./captcha.png")).await?;
    sleep(Duration::from_millis(2200)).await;
    captcha_pic.screenshot(Path::new("./captcha2.png")).await?;
    let rgba = open("./captcha.png").unwrap().into_rgba8();
    let _solution = solve_captcha(&rgba).await;

    print!(
        "false"
    );

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
#[show_image::main]
async fn main() {
    let account = Acc {
        accounttype: AccountType::Amazon,
        username: "test".to_string(),
        password: "password".to_string(),
        claimed: false,
    };
    let _result = claim(account).await;
    //    account.claimed=true;
}
