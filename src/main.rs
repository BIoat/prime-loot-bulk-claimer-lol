use thirtyfour::prelude::*;

mod captcha;
use captcha::*;

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

// #[macroquad::main("Captcha Solver [ manual ]")]
#[tokio::main]
async fn main() {

    captcha::open_window();
    loop {
        
    }
    // println!("Output is: {test}");
    let account = Acc {
        accounttype: AccountType::Amazon,
        username: "test".to_string(),
        password: "password".to_string(),
        claimed: false,
    };
    let result = claim(account).await;

}
