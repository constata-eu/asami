use api::AppConfig;
use std::process::Command;
use ethers::abi::Address;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Addresses {
  pub legacy: String,
  pub asami: String,
  pub doc: String,
  pub deployer: Address,
}

fn main() {
  let config = AppConfig::default_figment().expect("config to exist");

  println!("Killing existing ganaches");
  Command::new("bash")
    .args(&["-c", r#"pgrep -f "bin/ganache" | xargs -r kill -9"#])
    .output().expect("Could not kill previous server");

  let path_to_log = "/tmp/ganache.log";
  let dir = std::fs::canonicalize("../contract").unwrap();

  println!("Removing old ganache data");
  Command::new("rm").args(&["-rf", "../contract/ganache_data"]).output()
    .expect("Could not remove previous ganache_data dir");
  Command::new("mkdir").args(&["../contract/ganache_data"]).output()
    .expect("Could not remove previous ganache_data dir");

  println!("compiling contracts");
  Command::new("truffle").current_dir(&dir).args(["compile"]).output().unwrap();
    
  println!("compiling mockdoc");
  Command::new("truffle").current_dir(&dir).args(["compile", "test/MockDock.sol"]).output().unwrap();

  println!("running ganache");
  let child_vec = Command::new("ganache")
    .current_dir(&dir)
    .args([
      "-D",
      &format!("--logging.file={path_to_log}"),
      "--miner.instamine", "strict",
      "--miner.blockTime", "0",
      "--db", "./ganache_data",
      "-i", "12345",
      "--mnemonic", "correct battery horse staple"
    ])
    .output()
    .unwrap()
    .stdout;
  let child = String::from_utf8(child_vec).unwrap();

  println!("waiting for ganache to start");
  for _ in 0..100 {
    let status = ureq::get("http://127.0.0.1:8545").call();
    if let Err(ureq::Error::Status(404, _)) = status {
      break;
    }
  }

  println!("migrating");
  Command::new("truffle").current_dir(&dir).args(["migrate", "--network", "local"]).output().unwrap();

  println!("setting up blockchain data and balances");
  let output = Command::new("truffle")
    .current_dir(&dir)
    .env("ADMIN_ADDRESS", &config.rsk.admin_address)
    .env("MEMBER_ADDRESS", "0x6868db995fdEEf093320A8Ee64b01F450b044f2C")
    .args(["exec", "scripts/local_blockchain_state.js", "--network", "local"])
    .output()
    .unwrap();

  let out_str = String::from_utf8(output.stdout).unwrap();
  let json = out_str.lines().last().unwrap();
  serde_json::from_str::<Addresses>(json).expect("Local blockchain init script may have exited with an error.");
  std::fs::write("../contract/ganache_data/addresses.json", &json).expect("cannot write to ganache_data/addresses.json");
  println!("Done creating env, addresses were {json}");

  Command::new("ganache")
    .current_dir(&dir)
    .args(["instances", "stop", &child])
    .output()
    .unwrap();

  Command::new("bash")
    .args(&["-c", r#"pgrep -f "bin/ganache" | xargs -r kill -9"#])
    .output().expect("Could not kill previous server");

  for _ in 0..100 {
    let status = ureq::get("http://127.0.0.1:8545").call();
    if let Err(ureq::Error::Transport(_)) = status {
      break;
    }
  }
}
