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