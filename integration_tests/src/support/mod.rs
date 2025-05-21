// The TestHelper wraps a general TestApp, and also allows
// creating different kinds of users.
// It also contains all the shortcuts for easily establishing
// test scenarios without repetition.

pub mod selenium;
pub mod test_api_server;
pub mod test_app;
pub mod truffle;
pub mod vite_preview;
pub mod handle_scoring_builder;

use std::{
    io::{self, BufRead, Write},
    os::fd::AsRawFd,
    sync::Arc,
};

use nix::unistd::isatty;
pub use selenium::Selenium;
pub use test_user::{TestUser, ApiError};
pub use test_api_server::*;
pub use test_app::*;
use tokio::task::{self, JoinHandle};
pub use truffle::*;
pub use vite_preview::*;

pub mod test_user;
#[allow(unused_imports)]
pub use galvanic_assert::{
    self,
    matchers::{collection::*, variant::*, *},
    *,
};
pub use test_user::*;
pub use thirtyfour::{error::WebDriverResult, prelude::*, WebDriver, WebElement};

pub struct TestHelper {
    pub test_app: Arc<TestApp>,
}

impl TestHelper {
    pub async fn new() -> Self {
        Self { test_app: Arc::new(TestApp::init().await) }
    }

    pub async fn for_web() -> Self {
        Self { test_app: Arc::new(TestApp::init().await.with_web().await) }
    }

    pub async fn user(&self) -> TestUser {
        let mut u = TestUser::new(self.test_app.clone()).await;
        u.login_to_api_with_one_time_token().await;
        u
    }

    pub fn a(&self) -> &TestApp {
        &self.test_app
    }

    pub async fn advertiser(&self) -> TestUser {
        self.user().await.signed_up().await.advertiser().await
    }

    pub async fn collaborator(&self, score: i32) -> TestUser {
        self.user().await.signed_up().await.active(score).await
    }

    pub async fn stop(self) {
        Arc::try_unwrap(self.test_app).expect("could not stop test app").stop().await
    }

    pub fn web(&self) -> &Selenium {
        self.test_app.web.as_ref().unwrap()
    }
}
