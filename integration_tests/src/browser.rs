use std::{
    fs::File,
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Child, Command, Stdio},
    time::Duration,
};

use anyhow::{bail, Context};
use thirtyfour::prelude::*;

const FIXED_CHROME_URL: &str = "https://asami.ams3.digitaloceanspaces.com/chrome";

pub struct Browser {
    pub driver: WebDriver,
    child: Option<Child>,
}

impl Browser {
    pub async fn start(repository: &Path, url: &str, headless: bool) -> anyhow::Result<Self> {
        let fixture = repository.join("integration_tests/chromedrivers");
        let chrome = fixture.join("chrome-linux/chrome");
        let driver = fixture.join("chromedriver_linux");
        ensure_fixed_chrome(&chrome)?;

        let port = free_port()?;
        let log = File::create("/tmp/asami-chromedriver.log")?;
        let child = Command::new(&driver)
            .arg(format!("--port={port}"))
            .arg("--verbose")
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log))
            .spawn()
            .with_context(|| format!("start {}", driver.display()))?;

        wait_for_driver(port).await?;

        let mut capabilities = DesiredCapabilities::chrome();
        capabilities.set_binary(chrome.to_string_lossy().as_ref())?;
        let mut arguments = vec![
            "--no-sandbox".to_string(),
            "--disable-gpu".to_string(),
            "--disable-dev-shm-usage".to_string(),
            "--window-size=1440,1100".to_string(),
            "--disable-popup-blocking".to_string(),
        ];
        if headless {
            arguments.push("--headless=new".to_string());
        }
        capabilities.add_chrome_option("args", serde_json::to_value(arguments)?)?;

        let webdriver = WebDriver::new(&format!("http://127.0.0.1:{port}"), capabilities).await?;
        webdriver.goto(url).await?;
        Ok(Self {
            driver: webdriver,
            child: Some(child),
        })
    }

    pub async fn text(&self, selector: &str) -> anyhow::Result<String> {
        Ok(self
            .driver
            .query(By::Css(selector))
            .and_displayed()
            .first()
            .await
            .with_context(|| format!("find {selector}"))?
            .text()
            .await?)
    }

    pub async fn attribute(&self, selector: &str, name: &str) -> anyhow::Result<Option<String>> {
        Ok(self
            .driver
            .query(By::Css(selector))
            .and_displayed()
            .first()
            .await
            .with_context(|| format!("find {selector}"))?
            .attr(name)
            .await?)
    }

    pub async fn count(&self, selector: &str) -> anyhow::Result<usize> {
        Ok(self.driver.find_all(By::Css(selector)).await?.len())
    }

    pub async fn stop(mut self) {
        let _ = self.driver.clone().quit().await;
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn ensure_fixed_chrome(chrome: &Path) -> anyhow::Result<()> {
    let usable = std::fs::metadata(chrome).map(|metadata| metadata.len() > 300_000_000).unwrap_or(false)
        && Command::new(chrome)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
    if usable {
        return Ok(());
    }

    let status = Command::new("curl")
        .args(["-fL", FIXED_CHROME_URL, "-o"])
        .arg(chrome)
        .status()
        .context("download pinned Chromium executable")?;
    if !status.success() {
        bail!("could not download pinned Chromium executable");
    }

    let mut permissions = std::fs::metadata(chrome)?.permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(chrome, permissions)?;
    Ok(())
}

async fn wait_for_driver(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..100 {
        if client
            .get(format!("http://127.0.0.1:{port}/status"))
            .send()
            .await
            .map(|response| response.status().is_success())
            .unwrap_or(false)
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("ChromeDriver did not start")
}

fn free_port() -> anyhow::Result<u16> {
    Ok(TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port())
}
