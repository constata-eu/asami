use api::AppConfig;
use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};

pub struct Truffle {
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

impl Truffle {
    pub fn start() -> Self {
        let config = AppConfig::default_figment().expect("config to exist");
        let dir = std::fs::canonicalize("../contract").unwrap();
        Command::new("yarn")
            .args(["hardhat", "clean"])
            .current_dir(&dir)
            .output()
            .expect("Could not run hardhat clean");

        Command::new("yarn")
            .args(["hardhat", "compile"])
            .current_dir(&dir)
            .output()
            .expect("Could not run hardhat compile");

        let file = File::create("/tmp/latest_truffle_output.txt").unwrap();

        let child = unsafe {
            Command::new("yarn")
                .args(["hardhat", "node"])
                .stdout(Stdio::from(file)) // Redirect s
                .current_dir(&dir)
                .pre_exec(|| {
                    // Set the process group ID to the child's PID
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
        for _ in 0..100 {
            let status = ureq::get("http://127.0.0.1:8545").call();
            if status.is_ok() {
                running = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(running, "Local node not running.");

        Command::new("yarn")
            .args([
                "hardhat",
                "ignition",
                "deploy",
                "ignition/modules/Asami.js",
                "--network",
                "localhost",
            ])
            .current_dir(&dir)
            .env(
                "ADMIN_ADDRESS",
                ethers::utils::hex::encode(config.rsk.admin_address),
            )
            .env(
                "MEMBER_ADDRESS",
                "0x6868db995fdEEf093320A8Ee64b01F450b044f2C",
            )
            .output()
            .unwrap();

        let json = std::fs::read_to_string(
            "../contract/ignition/deployments/chain-31337/deployed_addresses.json",
        )
        .expect("cannot read deployed_addresses.json");
        let addresses: Addresses =
            serde_json::from_str(&json).expect("Wrong addresses.json file contents");

        let deployer = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

        Truffle {
            child,
            dir,
            addresses,
            deployer,
        }
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
        killpg(pid, Signal::SIGTERM).expect("Failed to kill process group");

        let mut stopped = false;
        for _ in 0..100 {
            let status = ureq::get("http://127.0.0.1:8545").call();
            if status.is_err() {
                stopped = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(stopped, "Local node did not stop.");
    }
}

impl std::ops::Drop for Truffle {
    fn drop(&mut self) {
        self.stop();
    }
}
