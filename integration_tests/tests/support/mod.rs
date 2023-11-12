pub mod selenium;
pub mod test_api_server;
pub mod vite_preview;
pub mod test_app;
pub mod truffle;

pub use selenium::Selenium;
pub use test_app::*;
pub use truffle::*;
pub use test_api_server::*;
pub use vite_preview::*;

pub mod test_api_client;
pub use test_api_client::*;

pub use thirtyfour::{
  error::WebDriverResult,
  WebDriver,
  WebElement,
  prelude::*
};
pub use galvanic_assert::{
  self,
  matchers::{collection::*, variant::*, *},
  *,
};

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

    #[test]
    fn $i() {
      use crate::support::*;

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
  ($i:ident($c:ident, $driver:ident) $($e:tt)* ) => {
    test!{ $i
      time_test::time_test!("integration test");
      let $driver = Selenium::start().await;

      let $c = crate::support::TestApp::init().await;
      let app = $c.app.clone();
      let mut vite_preview = VitePreview::start();
      let server = TestApiServer::start(app).await;

      {$($e)*};

      server.await.unwrap();
      vite_preview.stop();
      $driver.stop().await;
    }
  }
}

#[macro_export]
macro_rules! api_test {
  ($test_name:ident($test_app:ident, $client:ident) $($e:tt)* ) => {
    test!{ $test_name
      time_test::time_test!("api test");
      let $test_app = crate::support::TestApp::init().await;
      let $client = crate::support::ApiClient::new($test_app.clone()).await;
      {$($e)*};
    }
  }
}
