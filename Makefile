
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
	docker run -e PRIVATE_KEY=$$PRIVATE_KEY -e CONFIG_FILE=$(CONFIG_FILE) $(IMAGE_NAME)