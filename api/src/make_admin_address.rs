use std::io::{stdin, Read};

use ethers::{
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder},
};

fn main() {
    let mut password = String::new();
    stdin().read_to_string(&mut password).expect("Could not get password from stdin");
    password.pop();

    let mut rng = rand::thread_rng();
    MnemonicBuilder::<English>::default()
        .write_to(".")
        .password(&password)
        .word_count(24)
        .build_random(&mut rng)
        .expect("Could not build wallet");

    println!("Done, wrote mnemonic.");
}
