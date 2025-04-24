pub mod selenium;
pub mod test_api_server;
pub mod test_app;
pub mod truffle;
pub mod vite_preview;

use std::{
    io::{self, BufRead, Write},
    os::fd::AsRawFd,
    sync::Arc,
};

use nix::unistd::isatty;
pub use selenium::Selenium;
pub use test_api_client::{ApiClient, ApiError};
pub use test_api_server::*;
pub use test_app::*;
use tokio::task::{self, JoinHandle};
pub use truffle::*;
pub use vite_preview::*;

pub mod test_api_client;
#[allow(unused_imports)]
pub use galvanic_assert::{
    self,
    matchers::{collection::*, variant::*, *},
    *,
};
pub use test_api_client::*;
pub use thirtyfour::{error::WebDriverResult, prelude::*, WebDriver, WebElement};

#[allow(dead_code)]
pub fn wait_here() {
    use std::{thread, time};
    println!("Waiting here as instructed. ctrl+c to quit.");
    let ten_millis = time::Duration::from_millis(10);
    loop {
        thread::sleep(ten_millis);
    }
}

#[allow(dead_code)]
pub async fn wait_for_enter(msg: &'static str) {
    task::spawn_blocking(move || {
        if !isatty(io::stdin().as_raw_fd()).unwrap_or(false) {
            println!("[Skipping pause — not a TTY]");
            return;
        }

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut handle = stdin.lock();
        let mut buf = String::new();

        write!(stdout, "[{msg}] Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        handle.read_line(&mut buf).unwrap();
    })
    .await
    .unwrap();
}

pub async fn try_until<T: std::future::Future<Output = bool>>(times: i32, sleep: u64, err: &str, call: impl Fn() -> T) {
    assert!(wait_for(times, sleep, call).await, "{err}");
}

pub async fn wait_for<T: std::future::Future<Output = bool>>(times: i32, sleep: u64, call: impl Fn() -> T) -> bool {
    use std::{thread, time};
    let millis = time::Duration::from_millis(sleep);
    for _i in 0..times {
        if call().await {
            return true;
        }
        thread::sleep(millis);
    }
    false
}

#[allow(dead_code)]
pub fn pause_a_bit() {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(2000));
}

#[allow(dead_code)]
pub fn rematch<'a>(expr: &'a str) -> Box<dyn Matcher<'a, String> + 'a> {
    Box::new(move |actual: &String| {
        let re = regex::Regex::new(expr).unwrap();
        let builder = MatchResultBuilder::for_("rematch");
        if re.is_match(actual) {
            builder.matched()
        } else {
            builder.failed_because(&format!("{:?} does not match {:?}", expr, actual))
        }
    })
}

#[macro_export]
macro_rules! test {
  ($i:ident $($e:tt)* ) => {
    #[test_log::test(test)]
    fn $i() {
      use $crate::support::*;
      use anyhow::*;

      async fn run_test() -> std::result::Result<(), anyhow::Error> {
        {$($e)*}
        Ok(())
      }

      let result = tokio::runtime::Runtime::new()
        .expect("could not build runtime")
        .block_on(run_test());

      if let Err(e) = result {
        let source = e.source().map(|e| e.to_string() ).unwrap_or_else(|| "none".to_string());
        println!("Error: {e:?}\n Source: {source}.");
        panic!("Error in test. see backtrace");
      }
    }
  }
}

#[macro_export]
macro_rules! browser_test {
  ($i:ident(mut $browser:ident) $($e:tt)* ) => {
    test!{ $i
      time_test::time_test!("integration test");

      let test_app = $crate::support::TestApp::init().await;
      let server = TestApiServer::start(test_app.app.clone()).await;
      let mut vite_preview = VitePreview::start();
      let api = test_app.client().await;

      #[allow(unused_mut)]
      let mut $browser = Selenium::start(api).await;
      {$($e)*};

      server.abort();
      assert!(server.await.unwrap_err().is_cancelled());
      vite_preview.stop();
      $browser.stop().await;
    }
  }
}

#[macro_export]
macro_rules! api_test {
  ($test_name:ident(mut $client:ident) $($e:tt)* ) => {
    test!{ $test_name
      time_test::time_test!("api test");
      let app = $crate::support::TestApp::init().await;
      #[allow(unused_mut)]
      let mut $client = app.client().await;
      {$($e)*};
    }
  }
}

#[macro_export]
macro_rules! app_test {
  ($test_name:ident($test_app:ident) $($e:tt)* ) => {
    test!{ $test_name
      time_test::time_test!("app test");
      let $test_app = $crate::support::TestApp::init().await;
      {$($e)*};
    }
  }
}

pub struct TestHelper {
    pub app: Arc<TestApp>,
    pub api_server: Option<JoinHandle<()>>,
    pub vite_preview: Option<VitePreview>,
    pub api: Option<ApiClient>,
    pub web: Option<Selenium>,
}

impl TestHelper {
    pub async fn for_web() -> Self {
        let app = Arc::new(TestApp::init().await);
        let api_server = TestApiServer::start(app.app.clone()).await;
        let vite_preview = VitePreview::start();

        let mut this = Self {
            api_server: Some(api_server),
            vite_preview: Some(vite_preview),
            app,
            api: None,
            web: None,
        };

        let api = this.make_api_client().await;
        this.web = Some(Selenium::start(api).await);

        this
    }

    pub async fn for_model() -> Self {
        let app = Arc::new(TestApp::init().await);

        Self {
            api_server: None,
            vite_preview: None,
            app,
            api: None,
            web: None,
        }
    }

    pub async fn make_api_client(&self) -> ApiClient {
        let mut api = ApiClient::new(self.app.clone()).await;
        api.login().await;
        api
    }

    pub fn web(&self) -> &Selenium {
        self.web.as_ref().unwrap()
    }

    pub async fn stop(self) {
        if let Some(mut x) = self.vite_preview {
            x.stop();
        }
        if let Some(x) = self.web {
            x.stop().await;
        }
        if let Some(x) = self.api_server {
            x.abort();
            assert!(x.await.unwrap_err().is_cancelled());
        }
    }
}
