#[macro_use]
mod support;
//use ::api::models::*;

browser_test!{ logs_in_with_facebook (mut d)
  d.goto("http://127.0.0.1:5173/").await;
  wait_here();
}