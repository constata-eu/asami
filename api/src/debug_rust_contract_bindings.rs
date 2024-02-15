use ethers::prelude::Abigen;

/// Abigen is used to generate Rust code to interact with smart contracts on the blockchain.
/// It provides a way to encode and decode data that is passed to and from smart contracts.
/// The output of abigen is Rust code, that is bound to the contract's interface, allowing
/// developers to call its methods to read/write on-chain state and subscribe to realtime events.
///
/// The abigen tool can be used in two ways, addressing different use-cases scenarios and developer
/// taste:
///
/// 1. **Rust file generation:** takes a smart contract's Application Binary Interface (ABI)
/// file and generates a Rust file to interact with it. This is useful if the smart contract is
/// referenced in different places in a project. File generation from ABI can also be easily
/// included as a build step of your application.
/// 2. **Rust inline generation:** takes a smart contract's solidity definition and generates inline
/// Rust code to interact with it. This is useful for fast prototyping and for tight scoped
/// use-cases of your contracts.
/// 3. **Rust inline generation from ABI:** similar to the previous point but instead of Solidity
/// code takes in input a smart contract's Application Binary Interface (ABI) file.
#[tokio::main]
async fn main() {
  rust_file_generation();
}

fn rust_file_generation() {
  let abi_source = "../contract/build/contracts/Asami.json";
  let out_file = std::env::temp_dir().join("asami_contract.rs");
  if out_file.exists() {
    std::fs::remove_file(&out_file).unwrap();
  }
  Abigen::new("ASAMI", abi_source)
    .expect("abigen")
    .add_derive("serde::Deserialize")
    .expect("add_derive")
    .add_derive("serde::Serialize")
    .expect("add_derive")
    .generate()
    .expect("generate")
    .write_to_file(out_file)
    .expect("write to file");
}
