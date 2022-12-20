use thirtyfour::prelude::*;
async fn claim(account: Acc) -> WebDriverResult<()> {
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
    amazon_e.send_keys(account.username).await?;

    let amazon_p = driver.query(By::Id("ap_password")).first().await?;
    amazon_p.send_keys(account.password).await?;

    driver
        .query(By::Id("signInSubmit"))
        .first()
        .await?
        .click()
        .await?;

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
    let result = claim(account).await;
    //    account.claimed=true;
}
