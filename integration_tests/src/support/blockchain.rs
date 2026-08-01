use std::{
    fs::File,
    os::unix::process::CommandExt,
    process::{Child, Command, Stdio},
};

use api::AppConfig;
use nix::{
    sys::signal::{killpg, Signal},
    unistd::Pid,
};

const LOCAL_CHAIN_MNEMONIC: &str = "test test test test test test test test test test test junk";

#[derive(Debug)]
pub struct Blockchain {
    pub child: Child,
    pub dir: std::path::PathBuf,
    pub addresses: Addresses,
    pub deployer: &'static str,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Addresses {
    #[serde(rename = "LocalAsami#Asami")]
    pub legacy: String,
    #[serde(rename = "LocalAsami#AsamiCore")]
    pub asami: String,
    #[serde(rename = "LocalAsami#MockDoc")]
    pub doc: String,
}

impl Blockchain {
    pub fn start() -> Self {
        let config = AppConfig::default_figment().expect("config to exist");
        let dir = std::fs::canonicalize(format!("{}/../contract", env!("CARGO_MANIFEST_DIR"))).unwrap();
        Self::ensure_requirements(&dir);
        Self::ensure_compiled_artifacts(&dir);
        Self::ensure_build_info(&dir);

        let deployments_dir = dir.join("ignition/deployments/chain-31337");
        let _ = std::fs::remove_dir_all(&deployments_dir);

        let file = File::create("/tmp/latest_blockchain_output.txt").unwrap();
        let contract_dir = dir.clone();

        let child = unsafe {
            Command::new("anvil")
                .args([
                    "--host",
                    "127.0.0.1",
                    "--port",
                    "8545",
                    "--chain-id",
                    "31337",
                    "--mnemonic",
                    LOCAL_CHAIN_MNEMONIC,
                ])
                .stdout(Stdio::from(file)) // Redirect s
                .stderr(Stdio::from(File::options().append(true).open("/tmp/latest_blockchain_output.txt").unwrap()))
                .current_dir(&contract_dir)
                .pre_exec(|| {
                    nix::unistd::setsid()
                        .map_err(|e| {
                            eprintln!("Failed to create new process group: {:?}", e);
                            std::io::Error::from_raw_os_error(e as i32)
                        })
                        .unwrap();
                    Ok(())
                })
                .spawn()
                .expect("Could not run hardhat node")
        };

        let mut running = false;
        for _ in 0..200 {
            let rpc_ready = ureq::post("http://127.0.0.1:8545")
                .send_json(ureq::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_chainId",
                    "params": [],
                    "id": 1
                }))
                .ok()
                .and_then(|resp| resp.into_json::<serde_json::Value>().ok())
                .and_then(|json| json.get("result").cloned())
                .is_some();
            if rpc_ready {
                running = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(running, "Local node not running.");
        std::thread::sleep(std::time::Duration::from_millis(500));

        Self::run_yarn_env(
            &dir,
            &[
                "hardhat",
                "ignition",
                "deploy",
                "ignition/modules/Asami.js",
                "--network",
                "localhost",
                "--reset",
            ],
            &[
                ("ADMIN_ADDRESS", format!("{:#x}", config.rsk.admin_address)),
                ("MEMBER_ADDRESS", "0xbe992ec27E90c07caDE70c6C3CD26eECC8CadCfE".to_string()),
            ],
            "Could not deploy LocalAsami ignition module",
        );

        let json = std::fs::read_to_string(format!(
            "{}/ignition/deployments/chain-31337/deployed_addresses.json",
            dir.display()
        ))
        .expect("cannot read deployed_addresses.json");
        let addresses: Addresses = serde_json::from_str(&json).expect("Wrong addresses.json file contents");

        let deployer = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

        Blockchain {
            child,
            dir,
            addresses,
            deployer,
        }
    }

    fn ensure_requirements(dir: &std::path::Path) {
        assert!(
            Command::new("yarn")
                .arg("--version")
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            "yarn is required to run the local blockchain environment"
        );
        assert!(
            Command::new("anvil")
                .arg("--version")
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            "anvil must be installed before running the local blockchain environment"
        );
        assert!(
            dir.join("node_modules/.bin/hardhat").exists(),
            "contract/node_modules/.bin/hardhat is missing; install contract dependencies first"
        );
    }

    fn ensure_compiled_artifacts(dir: &std::path::Path) {
        for path in [
            "artifacts/contracts/Asami.sol/Asami.json",
            "artifacts/contracts/AsamiCore.sol/AsamiCore.json",
            "artifacts/contracts/MockDoc.sol/MockDoc.json",
        ] {
            assert!(
                dir.join(path).exists(),
                "missing compiled contract artifact at contract/{path}; compile the contract project before running run-dev"
            );
        }
    }

    fn ensure_build_info(dir: &std::path::Path) {
        assert!(
            dir.join("artifacts/build-info").exists(),
            "missing Hardhat build-info at contract/artifacts/build-info; run `cd contract && yarn hardhat compile --force` once before running run-dev"
        );
    }

    fn run_yarn_env(dir: &std::path::Path, args: &[&str], envs: &[(&str, String)], context: &str) {
        let mut cmd = Command::new("yarn");
        cmd.args(args).current_dir(dir);
        for (key, value) in envs {
            cmd.env(key, value);
        }
        let output = cmd.output().unwrap_or_else(|e| panic!("{context}: {e}"));
        assert!(
            output.status.success(),
            "{context}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    pub fn stop(&mut self) {
        match self.child.kill() {
            Ok(_) => println!("Child process terminated."),
            Err(e) => eprintln!("Failed to terminate child process: {}", e),
        }

        // Optionally, wait for the process to exit to clean up resources
        match self.child.wait() {
            Ok(status) => println!("Child process exited with status: {}", status),
            Err(e) => eprintln!("Failed to wait for child process: {}", e),
        }

        // To kill the entire process group:
        let pid = Pid::from_raw(self.child.id() as i32);
        if let Err(e) = killpg(pid, Signal::SIGTERM) {
            eprintln!("Failed to kill process group: {}", e);
        }

        let mut stopped = false;
        for _ in 0..100 {
            let rpc_ready = ureq::post("http://127.0.0.1:8545")
                .send_json(ureq::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_chainId",
                    "params": [],
                    "id": 1
                }))
                .ok()
                .and_then(|resp| resp.into_json::<serde_json::Value>().ok())
                .and_then(|json| json.get("result").cloned())
                .is_some();
            if !rpc_ready {
                stopped = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(stopped, "Local node did not stop.");
    }
}

impl std::ops::Drop for Blockchain {
    fn drop(&mut self) {
        self.stop();
    }
}
