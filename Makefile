API = cd api/
FDY = cd foundry/
RPC_URL = http://127.0.0.1:8545
PRI_KEY = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

.PHONY: install-rust
# Install Rust using rustup.
install-rust:
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

.PHONY: install-foundry
# Install Foundry.
install-foundry:
	@curl -L https://foundry.paradigm.xyz | bash

.PHONY: anvil
# Run Anvil.
anvil:
	@anvil

.PHONY: test
# Test smart contract.
test:
	@$(FDY) && forge test

.PHONY: compile
# Compile smart contract.
compile:
	@$(FDY) && forge build

.PHONY: deploy
# Deploy smart contract.
deploy:
	@$(FDY) && forge create --rpc-url $(RPC_URL) --private-key $(PRI_KEY) src/Cert.sol:Cert --broadcast

.PHONY: run
# Run the application.
run:
	@$(API) && cargo run	

.PHONY: listen
# Listen for events.
listen:
	@$(API) && cargo run	--bin listener

.PHONY: fmt
# Format the Rust files.
fmt:
	@$(API) && cargo fmt

help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf " - \033[36m%-20s\033[0m %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
