# Docker image name
IMAGE_NAME = solver

# Docker run options
CONFIG_FILE = ./config/config.json

.PHONY: docker_build
docker_build:
	docker build -t $(IMAGE_NAME) .

.PHONY: read_env_vars
read_env_vars:
	@if [ -z "$$PRIVATE_KEY" ]; then \
		read -p "Enter PRIVATE_KEY: " PRIVATE_KEY; \
	fi; \
	if [ -z "$$SEPOLIA_RPC_URL" ]; then \
		read -p "Enter SEPOLIA_RPC_URL: " SEPOLIA_RPC_URL; \
	fi; \
	if [ -z "$$SEPOLIA_WS_URL" ]; then \
		read -p "Enter SEPOLIA_WS_URL: " SEPOLIA_WS_URL; \
	fi; \
	if [ -z "$$FUJI_RPC_URL" ]; then \
		read -p "Enter FUJI_RPC_URL: " FUJI_RPC_URL; \
	fi; \
	if [ -z "$$FUJI_WS_URL" ]; then \
		read -p "Enter FUJI_WS_URL: " FUJI_WS_URL; \
	fi

.PHONY: docker_run_spoke_chain_caller
docker_run_spoke_chain_caller: read_env_vars
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) $(IMAGE_NAME) spoke-chain-caller

.PHONY: docker_run_intent_book_matchmaker
docker_run_intent_book_matchmaker: read_env_vars
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) $(IMAGE_NAME) intent-book-matchmaker

.PHONY: docker_run_swap_intent_settler
docker_run_swap_intent_settler: read_env_vars
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) -e SEPOLIA_RPC_URL=$$SEPOLIA_RPC_URL -e SEPOLIA_WS_URL=$$SEPOLIA_WS_URL -e FUJI_RPC_URL=$$FUJI_RPC_URL -e FUJI_WS_URL=$$FUJI_WS_URL -e RUST_LOG=khalani_solver=debug,swap_intent_settler=debug $(IMAGE_NAME) "swap-intent-settler --private-key $$PRIVATE_KEY --config-file $(CONFIG_FILE)"



.PHONY: docker_run_cross_chain_market_maker
docker_run_cross_chain_market_maker:
	@read -p "Enter PRIVATE_KEY: " PRIVATE_KEY; \
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) -it --rm $(IMAGE_NAME) "cross-chain-market-maker --private-key $(PRIVATE_KEY) --config-file $CONFIG_FILE"

.PHONY: run_swap_intent_settler
run_swap_intent_settler:
	RUST_LOG=khalani_solver=debug,swap_intent_settler=debug cargo run --package swap-intent-settler -- --private-key "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff" --config-file ./config/config.json