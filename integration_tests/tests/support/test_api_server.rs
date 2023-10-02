use std::process::{Child, Command, Stdio};

pub struct ApiServer(Child);

impl ApiServer {
  pub fn start() -> Self {
    Command::new("killall")
      .args(&["-9", "public_api"])
      .output().expect("Could not kill previous server");

    let args = if std::env::var("CI").is_ok() {
      vec!["run", "--release", "-p", "api", "--bin", "api"]
    } else {
      vec!["run", "-p", "api", "--bin", "api"]
    };

    let path_to_log = "/tmp/asami_api_server.log";
    Command::new("rm").args(&["-rf", path_to_log]).output()
      .expect("Could not remove previous log");

    let output_file = std::fs::File::create(path_to_log).unwrap();

    let child = Command::new("cargo")
      .current_dir(std::fs::canonicalize("../api").unwrap())
      .stdout(Stdio::from(output_file.try_clone().unwrap()))
      .stderr(Stdio::from(output_file))
      .args(&args)
      .spawn().unwrap();

    for i in 0..100 {
      let status = ureq::get("http://localhost:8000").call();
      if status.is_ok() {
        break;
      }
      std::thread::sleep(std::time::Duration::from_millis(500));
      if i == 99 && std::env::var("CI").is_err() {
        assert!(false, "Api server is taking too long to compile.");
      }
    }

    ApiServer(child)
  }

  pub fn stop(&mut self) {
    self.0.kill().unwrap();
  }
}

impl std::ops::Drop for ApiServer {
  fn drop(&mut self) {
    self.stop();
  }
}

