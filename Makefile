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
deploy/store-code/bond-account:
	$(DAEMON) tx wasm store artifacts/bondex_bond_account.wasm \
		--from $(FROM) \
		--chain-id $(CHAIN_ID) \
		--node $(NODE_URL) \
		--gas auto \
		--gas-adjustment $(GAS_ADJ) \
		--fees $(FEES) \
		--keyring-backend=test \
		-y

# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
deploy/store-code/cw20:
	$(DAEMON) tx wasm store cw20_base.wasm \
		--from $(FROM) \
		--chain-id $(CHAIN_ID) \
		--node $(NODE_URL) \
		--gas auto \
		--gas-adjustment $(GAS_ADJ) \
		--fees $(FEES) \
		--keyring-backend=test \
		-y

# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
deploy/store-code/cw721-fixed-price:
	$(DAEMON) tx wasm store nft_bond.wasm \
		--from $(FROM) \
		--chain-id $(CHAIN_ID) \
		--node $(NODE_URL) \
		--gas auto \
		--gas-adjustment $(GAS_ADJ) \
		--fees $(FEES) \
		--keyring-backend=test \
		-y

# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
deploy/store-code/cw721-base:
	$(DAEMON) tx wasm store cw721_base.wasm \
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
deploy/instantiate/bond_account:
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
execute/bond-account/issue-bond-series:
	$(DAEMON) tx wasm execute $(CONTRACT_ADDR) '$(ISSUE_BOND_SERIES_MSG)' \
    --from $(FROM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --gas auto \
    --gas-adjustment $(GAS_ADJ) \
    --fees $(FEES) \
    --keyring-backend=test \
    -y

BUY_BOND_MSG := $(shell cat configs/buy_bond_msg.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
execute/cw20/buy-bond:
	$(DAEMON) tx wasm execute $(CONTRACT_CW20_ADDR) '$(BUY_BOND_MSG)' \
    --from $(FROM2) \
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
query/bond-account/get-config:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_ADDR) '$(GET_CONFIG_QUERY)' \
    --node $(NODE_URL)

query/cw721-fixed-price/get-config:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW721_FIXED_PRICE_ADDR) '$(GET_CONFIG_QUERY)' \
    --node $(NODE_URL)

query/cw721-base/get-config:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW721_BASE_ADDR) '$(GET_CONFIG_QUERY)' \
    --node $(NODE_URL)

ALL_TOKENS_QUERY := $(shell cat configs/all_tokens_query.json)
query/cw721-base/all_tokens:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW721_BASE_ADDR) '$(ALL_TOKENS_QUERY)' \
    --node $(NODE_URL)

query/cw20/balance-bob:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW20_ADDR) '{"balance":{"address":"$(BOB)"}}' \
    --node $(NODE_URL)

query/cw20/balance-alice:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW20_ADDR) '{"balance":{"address":"$(ALICE)"}}' \
    --node $(NODE_URL)

query/cw20/balance-cw721-fixed-price:
	$(DAEMON) query wasm contract-state smart $(CONTRACT_CW20_ADDR) '{"balance":{"address":"$(CONTRACT_CW721_FIXED_PRICE_ADDR)"}}' \
    --node $(NODE_URL)

query/bank/balance-alice:
	$(DAEMON) query bank balance $(ALICE) $(DENOM) \
    --node $(NODE_URL)

query/bank/balance-bob:
	$(DAEMON) query bank balance $(BOB) $(DENOM) \
    --node $(NODE_URL)


query/bank/balance-bond-account:
	$(DAEMON) query bank balance $(CONTRACT_ADDR) $(DENOM) \
    --node $(NODE_URL)


#CONTRACT_ADDR
#BOB
query/bank/send:
	$(DAEMON) tx bank send $(ALICE) $(CONTRACT_ADDR) 10034580$(DENOM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --fees $(FEES) \
    --gas auto --gas-adjustment $(GAS_ADJ) \
    --keyring-backend test \
    -y


query/cw20/transfer:
	$(DAEMON) tx wasm execute $(CONTRACT_CW20_ADDR) \
    '{"transfer": {"recipient": "$(BOB)", "amount": "1000"}}' \
    --from $(ALICE) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --fees $(FEES) \
    --gas auto --gas-adjustment $(GAS_ADJ) \
    --keyring-backend test \
    -y

WITHDRAW_MSG := $(shell cat configs/withdraw_msg.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
execute/bond-account/withdraw:
	$(DAEMON) tx wasm execute $(CONTRACT_ADDR) '$(WITHDRAW_MSG)' \
    --from $(FROM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --gas auto \
    --gas-adjustment $(GAS_ADJ) \
    --fees $(FEES) \
    --keyring-backend=test \
    -y

PAYOUT_BONDS_MSG := $(shell cat configs/payout_bonds_msg.json)
# Ensure config.mk exists and has valid values.
# If missing, run `make setup` to create it from config.mk.example, then update values manually.
execute/bond-account/payout-bonds:
	$(DAEMON) tx wasm execute $(CONTRACT_ADDR) '$(PAYOUT_BONDS_MSG)' \
    --from $(FROM) \
    --chain-id $(CHAIN_ID) \
    --node $(NODE_URL) \
    --gas auto \
    --gas-adjustment $(GAS_ADJ) \
    --fees $(FEES) \
    --keyring-backend=test \
    -y
