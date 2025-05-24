API := 'api'
FDY := 'foundry'
RPC_URL := 'http://127.0.0.1:8545'
PRI_KEY := '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'

# Display recipes.
default:
    @just --list

# Install Rust using rustup.
install-rust:
    @curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Foundry.
install-foundry:
    @curl -L https://foundry.paradigm.xyz | bash

# Run Anvil.
anvil:
    @anvil

# Test smart contract.
test:
    @cd {{ FDY }} && forge test

# Compile smart contract.
compile:
    @cd {{ FDY }} && forge build

# Deploy smart contract.
deploy:
    @cd {{ FDY }} && forge create --rpc-url {{ RPC_URL }} --private-key {{ PRI_KEY }} src/Cert.sol:Cert --broadcast

# Run the application.
run:
    @cd {{ API }} && cargo run

# Listen for events.
listen:
    @cd {{ API }} && cargo run --bin listener

# Format the Rust files.
fmt:
    @cd {{ API }} && cargo fmt
