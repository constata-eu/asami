use std::process::Command;
use ethers::abi::Address;

#[derive(Clone)]
pub struct Truffle {
  pub child: String,
  pub dir: std::path::PathBuf,
  pub addresses: Addresses,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Addresses {
  pub legacy: String,
  pub asami: String,
  pub doc: String,
  pub deployer: Address,
}

impl Truffle {
  pub fn start() -> Self {
    Command::new("bash")
      .args(&["-c", r#"pgrep -f "bin/ganache" | xargs -r kill -9"#])
      .output().expect("Could not kill previous server");

    let path_to_log = "/tmp/ganache.log";
    Command::new("rm").args(&["-rf", path_to_log]).output()
      .expect("Could not remove previous log");

    let dir = std::fs::canonicalize("../contract").unwrap();

    Command::new("rm").args(&["-rf", "../contract/ganache_data_copy"]).output()
      .expect("Could not remove previous log");
    Command::new("cp").args(&["-r", "../contract/ganache_data", "../contract/ganache_data_copy"])
      .output().expect("Could not kill previous server");

    let child_vec = Command::new("ganache")
      .current_dir(&dir)
      .args([
        "-D",
        &format!("--logging.file={path_to_log}"),
        "--miner.instamine", "strict",
        "--miner.blockTime", "0",
        "--gasPrice=2000000000",
        "--db", "./ganache_data_copy",
        "-i", "12345",
        "--mnemonic", "correct battery horse staple"
      ])
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

    let json = std::fs::read_to_string("../contract/ganache_data/addresses.json").expect("cannot read ganache_data/addresses.json");
    let addresses: Addresses = serde_json::from_str(&json).expect("Wrong addresses.json file contents. Rebuild ganache state.");

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

