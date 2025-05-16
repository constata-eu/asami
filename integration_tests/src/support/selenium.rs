use std::{
    path::Path,
    process::{Child, Command},
};

use api::{lang, models};
use chrono::Utc;
use thirtyfour::prelude::*;

use super::ApiClient;

pub struct Selenium {
    pub driver: WebDriver,
    pub api: ApiClient,
    child: Child,
}

pub const DOWNLOADS: &str = "/tmp/asami-tests-downloads";
pub const DATA_DIR: &str = "/tmp/asami-browser-datadir";
pub const ARTIFACTS: &str = "/tmp/test-artifacts";
pub const METAMASK: &str = "/tmp/asami-test-metamask";
pub const EXTENSION_ID: &str = "nkbihfbeogaeaoehlefnkodbefgpgknn";
//pub const SEED: &str = "clay useful lion spawn drift census subway require small matrix guess away";
//pub const MEMBER_ADDR: "0xbe992ec27E90c07caDE70c6C3CD26eECC8CadCfE"

impl Selenium {
    pub async fn start(api: ApiClient) -> Selenium {
        let dir = std::fs::canonicalize(format!(
            "{}/../integration_tests/chromedrivers",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        Command::new("rm").args(["-r", "-f", DATA_DIR]).output().unwrap();
        Command::new("cp").args(["-r", &format!("{}/profile", dir.display()), DATA_DIR]).output().unwrap();

        Command::new("rm").args(["-r", "-f", METAMASK]).output().unwrap();
        Command::new("cp").args(["-r", &format!("{}/metamask", dir.display()), METAMASK]).output().unwrap();

        Command::new("rm").args(["-r", "-f", ARTIFACTS]).output().unwrap();
        std::fs::create_dir_all(ARTIFACTS).unwrap();

        Command::new("rm").args(["-r", "-f", DOWNLOADS]).output().unwrap();
        Command::new("mkdir").args(["-p", DOWNLOADS]).output().unwrap();

        let mut caps = DesiredCapabilities::chrome();
        caps.set_binary(&format!("{}/chrome-linux/chrome", dir.display())).unwrap();
        caps.add_chrome_option(
            "prefs",
            serde_json::json![{
              "download.default_directory": DOWNLOADS,
              "download": { "default_directory" : DOWNLOADS }
            }],
        )
        .unwrap();

        let driver_path = format!("{}/chromedriver_linux", dir.display());

        let data_dir_opt = format!("--user-data-dir={DATA_DIR}");
        let metamask_opt = format!("--load-extension={METAMASK}");
        let log_opt = format!("--log-path={ARTIFACTS}/chrome.log");
        let opts = vec![
            &data_dir_opt,
            &metamask_opt,
            "--no-default-browser-check",
            "--disable-component-update",
            "--no-sandbox",
            "--disable-gpu",
            "--window-size=1920,1080",
            "--disable-popup-blocking",
            "--enable-logging",
            "--v=1",
            &log_opt,
            "--disable-features=IsolateOrigins,site-per-process",
            "--disable-dev-shm-usage",
            "--disable-software-rasterizer",
            "--disable-site-isolation-trials",
            "--remote-debugging-port=9222",
            "--allow-insecure-localhost",
            "--ignore-certificate-errors",
            "--allow-file-access-from-files",
        ];

        caps.add_chrome_option("args", serde_json::to_value(opts).unwrap()).unwrap();

        Command::new("killall").args(["-9", &driver_path]).output().expect("Could not kill previous server");

        let child = Command::new(driver_path)
            .args(["--port=4444", "--verbose", &log_opt])
            .spawn()
            .expect("chromedriver did not start");

        loop {
            if ureq::get("http://localhost:4444/status").call().is_ok() {
                break;
            }
        }

        let driver = WebDriver::new("http://localhost:4444", caps).await.expect("Webdriver init");
        driver.maximize_window().await.expect("to maximize window");
        let selenium = Selenium { child, driver, api };

        selenium.go_to_extension_window("home.html").await;
        selenium.driver.close_window().await.unwrap();
        selenium.go_to_window("chrome://new-tab-page").await;
        selenium
    }

    pub fn app(&self) -> api::App {
        self.api.app()
    }

    pub fn test_app(&self) -> &super::TestApp {
        self.api.test_app.as_ref()
    }

    pub async fn wait_for(&self, selector: &str) -> WebElement {
        let elem = self
            .driver
            .query(By::Css(selector))
            .and_enabled()
            .and_displayed()
            .first()
            .await
            .unwrap_or_else(|_| panic!("{selector} not found"));

        elem.handle
            .execute(
                r#"arguments[0].scrollIntoView({block: "center", inline: "center"});"#,
                vec![elem.to_json().unwrap()],
            )
            .await
            .unwrap_or_else(|_| panic!("{selector} not scrolled into view"));
        elem
    }

    pub async fn wait_for_text(&self, selector: &str, regex: &str) -> WebElement {
        let re = regex::Regex::new(regex).unwrap_or_else(|_| panic!("invalid regex {regex}"));
        self.driver
            .query(By::Css(selector))
            .with_text(re)
            .and_displayed()
            .first()
            .await
            .unwrap_or_else(|_| panic!("Expecting selector {selector} with text {regex}"))
    }

    pub async fn wait_until_gone(&self, selector: &str) {
        let found = self.driver.query(By::Css(selector)).single().await;
        if let Err(thirtyfour::error::WebDriverError::NoSuchElement(_)) = found {
            return;
        }

        let gone = found
            .expect("No errors")
            .wait_until()
            .wait(
                std::time::Duration::from_millis(10000),
                std::time::Duration::from_millis(500),
            )
            .stale()
            .await;

        if gone.is_err() {
            let time = Utc::now();
            let target = format!("{ARTIFACTS}/{selector}_{time}.png");
            self.save_screenshot(&target).await.unwrap();
        }
        gone.unwrap_or_else(|_| panic!("{selector} was still displayed"));
    }

    pub async fn click(&self, selector: &str) {
        let elem = self.wait_for(selector).await;
        elem.wait_until().enabled().await.unwrap_or_else(|_| panic!("{selector} not clickable"));
        elem.wait_until().clickable().await.unwrap_or_else(|_| panic!("{selector} not clickable"));
        elem.click().await.unwrap_or_else(|_| panic!("{selector} clickable but not clicked, wtf."));
    }

    pub async fn fill_in(&self, selector: &str, value: &str) {
        self.fill_in_with_enter(selector, value, false).await
    }

    pub async fn fill_in_with_enter(&self, selector: &str, value: &str, send_enter: bool) {
        let elem = self
            .driver
            .query(By::Css(selector))
            .first()
            .await
            .unwrap_or_else(|_| panic!("{selector} not found"));
        elem.wait_until().enabled().await.unwrap_or_else(|_| panic!("{selector} was not enabled"));
        elem.send_keys(value).await.unwrap_or_else(|_| panic!("Error sending {value} to {selector}"));
        if send_enter {
            elem.send_keys(Key::Enter.to_string())
                .await
                .unwrap_or_else(|_| panic!("Error sending enter to {selector}"));
        }
    }

    pub async fn goto(&self, url: &str) {
        self.driver.goto(url).await.unwrap_or_else(|_| panic!("Could not visit {url}"));
    }

    pub async fn stop(mut self) {
        let _dontcare = self.driver.quit().await;
        let _dontcare_either = self.child.kill();
    }

    pub async fn signup_with_one_time_token(&self) {
        self.api
            .test_app
            .app
            .one_time_token()
            .insert(models::InsertOneTimeToken {
                value: "user-token-1".to_string(),
                lang: lang::Lang::Es,
                lookup_key: "one_time_token".to_string(),
                email: None,
                user_id: None,
            })
            .save()
            .await
            .unwrap();

        self.goto("http://127.0.0.1:5173/#/one_time_token_login?token=user-token-1").await;
        self.wait_for("#member-dashboard").await;
    }

    pub async fn login(&self) {
        let token = format!("web-login-{}", Utc::now().timestamp());

        self
            .api
            .test_app
            .app
            .one_time_token()
            .insert(models::InsertOneTimeToken {
                value: token.clone(),
                lang: lang::Lang::Es,
                lookup_key: token.clone(),
                email: None,
                user_id: None,
            })
            .save()
            .await
            .unwrap();

        self.api
            .test_app
            .app
            .auth_method()
            .insert(models::InsertAuthMethod {
                user_id: self.api.user().await.attrs.id,
                kind: models::AuthMethodKind::OneTimeToken,
                lookup_key: token.clone()
            })
            .save()
            .await
            .unwrap();

        self.goto("http://127.0.0.1:5173/").await;
        self.goto(&format!("http://127.0.0.1:5173/#/one_time_token_login?token={token}")).await;
        self.wait_for("#member-dashboard").await;
    }

    pub async fn goto_member_dashboard(&self) {
        self.goto("http://127.0.0.1:5173/#/?role=member").await;
        self.wait_for("#member-dashboard").await;
    }

    pub async fn link_wallet_and_sign_login(&self) {
        self.click(".rlogin-provider-icon img[alt=MetaMask]").await;

        self.go_to_extension_notification().await;

        self.fill_in("input[data-testid=unlock-password]", "password").await;
        self.click("button[data-testid=unlock-submit]").await;

        self.go_to_app_window().await;
        self.click("button.rlogin-button.confirm").await;

        self.confirm_wallet_action().await;
    }

    pub async fn confirm_wallet_action(&self) {
        self.go_to_extension_notification().await;
        self.click("button[data-testid=confirm-footer-button]").await;
        self.go_to_app_window().await;
    }

    pub async fn go_to_window(&self, prefix: &str) {
        use std::{thread, time};
        let millis = time::Duration::from_millis(100);
        let mut windows = vec![];
        for _i in 0..20 {
            windows = self.driver.windows().await.unwrap();
            for handle in &windows {
                self.driver.switch_to_window(handle.clone()).await.unwrap();
                let url = self.driver.current_url().await.unwrap();
                if url.to_string().starts_with(prefix) {
                    println!("Entering window {url}");
                    return;
                }
            }
            thread::sleep(millis);
        }

        println!("Open browser windows are:");
        for handle in windows {
            self.driver.switch_to_window(handle).await.unwrap();
            let url = self.driver.current_url().await.unwrap();
            println!("{url}");
        }
        panic!("Could not find {prefix}");
    }

    pub async fn go_to_extension_window(&self, page: &str) {
        self.go_to_window(&format!("chrome-extension://{EXTENSION_ID}/{page}")).await
    }

    pub async fn go_to_extension_notification(&self) {
        self.go_to_extension_window("notification").await
    }

    pub async fn go_to_app_window(&self) {
        self.go_to_window("http://127.0.0.1").await
    }

    pub async fn save_screenshot(&self, name: &str) -> WebDriverResult<()> {
        let timestamp = Utc::now().format("%Y%m%dT%H%M%S");
        let filename = format!("{ARTIFACTS}/{}-{}.png", name, timestamp);
        self.driver.screenshot(Path::new(&filename)).await?;
        println!("Saved screenshot to: {}", filename);
        Ok(())
    }
}
