-include configs/config.mk
.PHONY: setup bond-account-wasm schema optimize wasm unit-test integration-test multitest

bond-account-wasm:
	cd contracts/bondex-bond-account && cargo wasm

## Setup development environment (pre-commit, hooks, etc.)
setup:
	./scripts/init.sh

# Optimize the WASM binaries for smaller size and better performance
optimize:
	./scripts/optimize-wasm.sh

# Build WASM binary for smart contract deployment
wasm:
	cargo wasm

# Run unit tests for contracts
unit-test:
	cargo unit-test

# Run integration tests to verify contract interactions
integration-test:
	cargo integration-test

# Run multi-contract integration tests with cw-multi-test framework
multitest:
	cargo multitest

# Get a list of all member directories in the cargo workspace
MEMBER_DIRS := $(shell cargo metadata --no-deps --format-version 1 \
	| jq -r '. as $$m | $$m.packages[] | select(.id | IN($$m.workspace_members[])) | .manifest_path' \
	| xargs -n1 dirname)

# Generate JSON schema files for contract messages in all member directories
schema:
	@for dir in $(MEMBER_DIRS); do \
		echo "Generating schema in $$dir..."; \
		cd $$dir && rm -rf schema && mkdir -p schema && cargo run --bin schema; \
	done

# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
deploy/store-code:
	$(DAEMON) tx wasm store artifacts/bondex_bond_account.wasm \
		--from $(FROM) \
		--chain-id $(CHAIN_ID) \
		--node $(NODE_URL) \
		--gas auto \
		--gas-adjustment $(GAS_ADJ) \
		--fees $(FEES) \
		--keyring-backend=test \
		-y

INSTANTIATE_MSG := $(shell cat configs/instantiate_msg.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
deploy/instantiate:
	$(DAEMON) tx wasm instantiate $(CONTRACT_CODE_ID) '$(INSTANTIATE_MSG)' \
    --label "bondex_bond_account_instantiate_stub" \
    --from $(FROM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --no-admin \
    --gas auto \
    --gas-adjustment $(GAS_ADJ) \
    --fees $(FEES) \
    --keyring-backend=test \
    -y

ISSUE_BOND_SERIES_MSG := $(shell cat configs/issue_bond_series_msg.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
execute/issue-bond-series:
	$(DAEMON) tx wasm execute $(CONTRACT_ADDR) '$(ISSUE_BOND_SERIES_MSG)' \
    --from $(FROM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --gas auto \
    --gas-adjustment $(GAS_ADJ) \
    --fees $(FEES) \
    --keyring-backend=test \
    -y

GET_CONFIG_QUERY := $(shell cat configs/get_config_query.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
query/get-config:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_ADDR) '$(GET_CONFIG_QUERY)' \
    --node $(NODE_URL)
