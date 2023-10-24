use std::process::Command;

#[derive(Clone)]
pub struct Truffle {
  pub child: String,
  pub dir: std::path::PathBuf,
  pub addresses: Addresses,
}

#[derive(Clone, serde::Deserialize)]
pub struct Addresses {
  pub asami: String,
  pub doc: String,
}

impl Truffle {
  pub fn start(admin_address: &str) -> Self {
    Command::new("bash")
      .args(&["-c", "kill -9 $(ps -ef | grep [g]anache | awk '{print $2}')"])
      .output().expect("Could not kill previous server");

    let path_to_log = "/tmp/ganache.log";
    Command::new("rm").args(&["-rf", path_to_log]).output()
      .expect("Could not remove previous log");

    let dir = std::fs::canonicalize("../contract").unwrap();

    Command::new("truffle").current_dir(&dir).args(["compile"]).output().unwrap();
    Command::new("truffle").current_dir(&dir).args(["compile", "test/MockDock.sol"]).output().unwrap();

    let child_vec = Command::new("ganache")
      .current_dir(&dir)
      .args(["-D", &format!("--logging.file={path_to_log}")])
      .output()
      .unwrap()
      .stdout;

    let child = String::from_utf8(child_vec).unwrap();

    for _ in 0..100 {
      let status = ureq::get("http://127.0.0.1:8545").call();
      if let Err(ureq::Error::Status(404, _)) = status {
        break;
      }
    }

    Command::new("truffle").current_dir(&dir).args(["migrate", "--network", "local"]).output().unwrap();
    let out = Command::new("truffle")
      .current_dir(&dir)
      .env("ADMIN_ADDRESS", &admin_address)
      .env("BROWSER_ADDRESS", "0x68aC79AE5D9C9eaf6c783836C35bbcbaCAe7C771")
      .args(["exec", "scripts/local_blockchain_state.js", "--network", "local"])
      .output()
      .unwrap();

    let out_str = String::from_utf8(out.stdout).unwrap();
    let addresses: Addresses = serde_json::from_str(&out_str.lines().last().unwrap())
      .expect("Local blockchain init script may have exited with an error.");

    Truffle { child, dir, addresses }
  }

  pub fn stop(&mut self) {
    Command::new("ganache")
      .current_dir(&self.dir)
      .args(["instances", "stop", &self.child])
      .output()
      .unwrap();
  }
}

impl std::ops::Drop for Truffle {
  fn drop(&mut self) {
    self.stop();
  }
}

