enum AccountType {
    Amazon, 
    Twitch,
} 
struct Acc {
    accounttype: AccountType,
    username: String,
    password: String,
    claimed: bool,
}

use thirtyfour::prelude::*;
async fn amazon_login(amazonusername: &str, amazonpassword: &str) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Navigate to https://wikipedia.org.
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
    amazon_e.send_keys(amazonusername).await?;

    let amazon_p = driver.query(By::Id("ap_password")).first().await?;
    amazon_p.send_keys(amazonpassword).await?;

    driver
        .query(By::Id("signInSubmit"))
        .first()
        .await?
        .click()
        .await?;

    driver.quit().await?;

    Ok(())
}
#[tokio::main]
async fn main() {
    let account = Acc {
        accounttype: AccountType::Amazon,
        username: "test".to_string(),
        password: "password".to_string(),
        claimed: false,
    };
    let result = amazon_login(&account.username,&account.password).await;
    //    account.claimed=true;
    let result2 = amazon_login("wad", "wad").await;
    println!("{}", match account {
        username => "username existo",
        _ => "sexo testo"
    });
}
