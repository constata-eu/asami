use std::process::{Child, Command, Stdio};

pub struct VitePreview(Child);

impl VitePreview {
    pub fn start() -> Self {
        Command::new("killall").args(["-9", "yarn"]).output().expect("Could not kill previous server");

        let path_to_log = "/tmp/asami_vite.log";
        Command::new("rm").args(["-rf", path_to_log]).output().expect("Could not remove previous log");

        let output_file = std::fs::File::create(path_to_log).unwrap();

        let child = Command::new("yarn")
            .current_dir(std::fs::canonicalize("../pwa").unwrap())
            .stdout(Stdio::from(output_file.try_clone().unwrap()))
            .stderr(Stdio::from(output_file))
            .args(["dev"])
            .spawn()
            .unwrap();

        for i in 0..100 {
            let status = ureq::get("http://localhost:5173").call();
            if status.is_ok() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
            if i == 99 && std::env::var("CI").is_err() {
                panic!("Vite dev server is taking too long.");
            }
        }

        VitePreview(child)
    }

    pub fn stop(&mut self) {
        self.0.kill().unwrap();
    }
}

impl std::ops::Drop for VitePreview {
    fn drop(&mut self) {
        self.stop();
    }
}
