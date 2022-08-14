# aidl-smart-contract-rust
Thesis for Master in Artificial Intelligence and Deep Learning - Design and Implementation of a smart contract in a Blockchain Network to evaluate the characteristics of datasets.


## Basic Installation
- WSL: wsl --install
- Curl: sudo apt-get install curl
- NVM: curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash
- Restart
- NVM: nvm install --lts
- Rust: curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    - rustup --version
    - rustc --version
    - cargo --version
    - Run Validator: solana-test-validator
- Mocka: npm install -g mocha
- Yarn: npm install -g yarn

## Solana Installation
- Solana latest: sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
    - solana --version
    - solana config set --url localhost
    - solana config get

## Anchor Installation
- Anchor Vrsion Manager: cargo install --git https://github.com/project-serum/anchor avm --locked --force
- Update Linux Packages: sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
- Restart
- Anchor: avm install latest
    - avm use latest
    - anchor --version

## Projact Initialization
- anchor init aidl-smart-contract-rust --javascript
    - Navigate to project: cd aidl-smart-contract-rust
    - Open your Favourite editor (VSCode): code .
- Generate Local Keypair: solana-keygen new
    - solana address
- Compile, Deploy and Run Tests: anchor test

You are free to uild your amazing Web3 App!!