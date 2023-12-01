use std::process::{Child, Command, Stdio};
use super::ApiClient;
use chrono::Utc;
use thirtyfour::prelude::*;
use api::models;
use crate::support::{wait_here, pause_a_bit, try_until};

pub struct Selenium{
  pub driver: WebDriver,
  pub api: ApiClient,
  child: Child,
}

pub const DOWNLOADS: &str = "/tmp/asami_tests_downloads";

impl Selenium {
  pub async fn start(api: ApiClient) -> Self {
    Command::new("rm").args(&["-r", "-f", "/tmp/asami_browser_datadir"])
      .output().expect("Could not delete downloads link");
    Command::new("cp").args(&["-r", "chromedrivers/profile", "/tmp/asami_browser_datadir"])
      .output().expect("Could not copy profile folder to temp location");

    Command::new("rm").args(&["-r", "-f", DOWNLOADS])
      .output().expect("Could not delete downloads link");
    Command::new("mkdir").args(&["-p", DOWNLOADS])
      .output().expect("Could not create downloads dir");

    let mut caps = DesiredCapabilities::chrome();
    caps.add_extension(std::path::Path::new("chromedrivers/metamask.crx")).unwrap();
    caps.add_chrome_option(
      "prefs",
      serde_json::json![{
        "download.default_directory": DOWNLOADS,
        "download": { "default_directory" : DOWNLOADS }
      }]
    ).unwrap();
    caps.add_chrome_arg("--user-data-dir=/tmp/asami_browser_datadir").unwrap();

    let local_driver_path = format!("chromedrivers/chromedriver_{}", std::env::consts::OS);
    let driver_path: String;

    if std::env::var("CI").is_ok() {
      caps.add_chrome_option("args", serde_json::to_value(vec![
        "--headless",
        "--no-sandbox",
        "--disable-gpu",
        "--disable-dev-shm-usage",
        "--window-size=1920,1080",
      ]).unwrap()).unwrap();
      driver_path = "chromedriver".to_string();
    } else {
      driver_path = local_driver_path.clone();
    }

    Command::new("killall").args(&["-9", &driver_path])
      .output().expect("Could not kill previous server");

    let start_driver = |path|{ Command::new(&path).stderr(Stdio::null()).stdout(Stdio::null()).args(&["--port=4444"]).spawn() };

    let child = if let Ok(child) = start_driver(&driver_path) {
      child
    } else {
      start_driver(&local_driver_path).expect("At least local driver to exist in CI mode")
    };

    loop {
      if ureq::get("http://localhost:4444/status").call().is_ok() {
        break;
      }
    }

    let driver = WebDriver::new("http://localhost:4444", caps).await.expect("Webdriver init");
    driver.maximize_window().await.expect("to maximize window");
    Selenium{child, driver, api}
  }

  pub async fn wait_for(&self, selector: &str) -> WebElement {
    let elem = self.driver
      .query(By::Css(selector))
      .and_enabled()
      .and_displayed()
      .first().await.expect(&format!("{selector} not found"));

    elem.handle.execute(
      r#"arguments[0].scrollIntoView({block: "center", inline: "center"});"#,
      vec![elem.to_json().unwrap()]
    ).await.expect(&format!("{selector} not scrolled into view"));
    elem
  }

  pub async fn wait_for_text(&self, selector: &str, regex: &str) -> WebElement {
    let re = regex::Regex::new(regex).expect(&format!("invalid regex {regex}"));
    self.driver.query(By::Css(selector))
      .with_text(re)
      .and_displayed()
      .first().await.expect(&format!("Expecting selector {selector} with text {regex}"))
  }

  pub async fn wait_until_gone(&self, selector: &str) {
    let found = self.driver.query(By::Css(selector)).single().await;
    if let Err(thirtyfour::error::WebDriverError::NoSuchElement(_)) = found {
      return;
    }

    let gone = found.expect("No errors").wait_until()
      .wait(std::time::Duration::from_millis(10000), std::time::Duration::from_millis(500))
      .stale().await;

    if gone.is_err() {
      let time = Utc::now();
      let target = format!("artifacts/{selector}_{time}");
      self.driver.screenshot(std::path::Path::new(&target)).await.expect("to save screenshot");
    }
    gone.expect(&format!("{selector} was still displayed"));
  }

  /*
  pub async fn not_exists(&self, selector: &str) {
    self.driver.query(By::Css(selector)).not_exists().await.expect(&format!("{selector} exists"));
  }
  */

  pub async fn click(&self, selector: &str) {
    let elem = self.wait_for(selector).await;
    elem.wait_until().enabled().await.expect(&format!("{selector} not clickable"));
    elem.wait_until().clickable().await.expect(&format!("{selector} not clickable"));
    elem.click().await.expect(&format!("{selector} clickable but not clicked, wtf."));
  }

  pub async fn fill_in(&self, selector: &str, value: &str) {
    let elem = self.driver.query(By::Css(selector)).first().await.expect(&format!("{selector} not found"));
    elem.wait_until().enabled().await.expect(&format!("{selector} was not enabled"));
    elem.send_keys(value).await.expect(&format!("Error sending {value} to {selector}"));
  }

  /*
  pub async fn delete_letters_and_send_new_keys(&self, selector: &str, times: i32, new_keys: &str) {
    let element = self.driver.find(By::Css(selector)).await.expect("to find {selector} for deletion");
    for _ in 0..times {
      let _ = element.send_keys(Key::Backspace.to_string()).await;
    }
    self.driver.find(By::Css(selector)).await
      .expect("to find {selector} after deletion and possible redraw.")
      .send_keys(new_keys).await
      .expect(&format!("to fill in {selector}")); 
  }
  */

  /*
  pub async fn close_window_and_go_to_handle_zero(&self) {
    let handles = self.driver.windows().await.expect("to get the window handles");
    self.driver.close_window().await.expect("to close window");
    self.driver.switch_to_window(handles[0].clone()).await.expect("to switch window zero");
  }
  */

  pub async fn open_metamask(&self) {
    self.goto("chrome-extension://nkbihfbeogaeaoehlefnkodbefgpgknn/popup.html").await;
  }

  pub async fn goto(&self, url: &str) {
    self.driver.goto(url).await.expect(&format!("Could not visit {url}"));
  }

  pub async fn stop(mut self) {
    let _dontcare = self.driver.quit().await;
    let _dontcare_either = self.child.kill();
  }

  /*
  pub async fn select_dropdown_item(&self, selector: &str, i: i32) {
    self.click(selector).await;
    let mut chain = self.driver.action_chain();

    for _ in 0..i {
      chain = chain.key_down(thirtyfour::Key::Down)
      .key_up(thirtyfour::Key::Down);
    }

    chain.key_down(thirtyfour::Key::Enter)
      .key_up(thirtyfour::Key::Enter)
      .perform().await.expect("to select item sucessfully");
  }
  */

  pub async fn signup_with_one_time_token(&self) {
    self.api.test_app.app.one_time_token()
      .insert(models::InsertOneTimeToken{value: "advertiser-token".to_string() })
      .save().await.unwrap();

    self.goto("http://127.0.0.1:5173/#/one_time_token_login?token=advertiser-token").await;
    self.wait_for("#advertiser-dashboard").await;
  }

  pub async fn login(&mut self) {
    self.api.login().await;

    let token = format!("web-login-{}", Utc::now().timestamp());
    
    let one_time_token = self.api.test_app.app.one_time_token()
      .insert(models::InsertOneTimeToken{ value: token.clone(), })
      .save().await.unwrap();

    self.api.test_app.app.auth_method().insert(models::InsertAuthMethod{
      user_id: self.api.user().await.attrs.id,
      kind: models::AuthMethodKind::OneTimeToken,
      lookup_key: format!("one_time_token:{}", one_time_token.attrs.id),
    }).save().await.unwrap();

    self.goto(&format!("http://127.0.0.1:5173/")).await;
    self.goto(&format!("http://127.0.0.1:5173/#/one_time_token_login?token={token}")).await;
    self.wait_for("#advertiser-dashboard").await;
  }

  pub async fn goto_member_dashboard(&self) {
    self.goto("http://127.0.0.1:5173/#/?role=member").await;
    self.wait_for("#member-dashboard").await;
  }

  pub async fn link_wallet_and_sign_login(&self) -> anyhow::Result<()> {
    self.click(".rlogin-provider-icon img[alt=MetaMask]").await;

    try_until(10, "No other window opened", || async {
      self.driver.windows().await.unwrap().len() == 2
    }).await;

    let mut handles = self.driver.windows().await.unwrap();
    self.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");

    self.fill_in("input[data-testid=unlock-password]", "password").await;
    self.click("button[data-testid=unlock-submit]").await;
    self.click("button[data-testid=page-container-footer-next]").await;
    self.wait_for(".permission-approval-container__content-container").await;

    self.click("button[data-testid=page-container-footer-next]").await;

    self.driver.switch_to_window(handles[0].clone()).await.unwrap();
    self.click("button.rlogin-button.confirm").await;

    try_until(10, "No other window opened", || async {
      self.driver.windows().await.unwrap().len() == 2
    }).await;
    handles = self.driver.windows().await.unwrap();
    self.driver.switch_to_window(handles[1].clone()).await?;

    self.click("button[data-testid=page-container-footer-next]").await;

    self.driver.switch_to_window(handles[0].clone()).await?;
    Ok(())
  }
}
