# Rust-DApp-Example

DApp example of deploying and interacting with Solidity smart contract using Rust.

## 🛠 Built With

[![Rust Badge](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge)](https://www.rust-lang.org/)
[![Solidity Badge](https://img.shields.io/badge/Solidity-363636?logo=solidity&logoColor=fff&style=for-the-badge)](https://soliditylang.org/)
[![Foundry Badge](https://img.shields.io/badge/Foundry-3C3C3D?logo=ethereum&logoColor=fff&style=for-the-badge)](https://book.getfoundry.sh/)
[![Alloy Badge](https://img.shields.io/badge/Alloy-3C3C3D?logo=ethereum&logoColor=fff&style=for-the-badge)](https://alloy.rs/)
[![Axum Badge](https://img.shields.io/badge/Axum-000?logo=rust&logoColor=fff&style=for-the-badge)](https://docs.rs/axum/latest/axum/)

## ⚙️ Run Locally

Clone the project.

```bash
git clone https://github.com/tr1sm0s1n/rust-dapp-example.git
cd rust-dapp-example
```

Fetch Foundry submodule.

```bash
git submodule update --init --recursive
```

Install Rust.

```bash
make install-rust
```

Install Foundry.

```bash
make install-foundry
```

Test smart contract.

```bash
make test
```

Run Anvil.

```bash
make anvil
```

Compile smart contract.

```bash
make compile
```

Deploy smart contract.

```bash
make deploy
```

Run the application.

```bash
make run
```

Listen for events (new terminal).

```bash
make listen
```

Issue a certificate (new terminal).

```bash
curl -X POST http://localhost:3000/issue -H "Content-Type: application/json" -d '{"id": "101", "name": "Keith", "course": "ETH-Rust", "grade": "A", "date": "06-06-24"}'
```

Fetch a certificate.

```bash
curl http://localhost:3000/fetch/101
```

## 📜 License

Click [here](./LICENSE.md).

## 🎗️ Contributing

Click [here](./CONTRIBUTING.md).

## ⚖️ Code of Conduct

Click [here](./CODE_OF_CONDUCT.md).
