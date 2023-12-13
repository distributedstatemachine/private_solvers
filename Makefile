# Docker image name
IMAGE_NAME = solver

# Docker run options
CONFIG_FILE = config/config.json

.PHONY: docker_build
docker_build:
	docker build -t $(IMAGE_NAME) .

.PHONY: docker_run
docker_run:
	@read -p "Enter PRIVATE_KEY: " PRIVATE_KEY; \
	read -p "Enter SEPOLIA_RPC_URL: " SEPOLIA_RPC_URL; \
	read -p "Enter SEPOLIA_WS_URL: " SEPOLIA_WS_URL; \
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) -e SEPOLIA_RPC_URL=$$SEPOLIA_RPC_URL -e SEPOLIA_WS_URL=$$SEPOLIA_WS_URL $(IMAGE_NAME)

.PHONY: run_swap_intent_settler
run_swap_intent_settler:
	RUST_LOG=khalani_solver=debug,swap_intent_settler=debug cargo run --package swap-intent-settler -- --private-key "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff" --config-file ./config/config.json
