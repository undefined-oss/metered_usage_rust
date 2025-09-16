.PHONY: dev-third-party-services dev-all dev-start-api dev-parquet build-prod-api-image build-and-publish-prod-api-image start-again dev-container-start-api test-post dev-parquet-help help

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

check_github_container_token_env_var: # Check if the Github Token is present to publish a prod image from local
	@if [ -z "${CR_PAT}" ]; then \
	echo "Error: CR_PAT is not set."; \
		exit 1; \
	fi

check_image_tag_env_var: # Check if a Image Tag is defined to build a prod image in local
	@if [ -z "${IMAGE_TAG}" ]; then \
	echo "Error: IMAGE_TAG is not set."; \
		exit 1; \
	fi

dev-third-party-services: # Launch all services in the profile "third"
	docker compose --profile third up --watch

dev-all: # Launch all services in the profile "all" on mode watch
	docker compose --profile all up --watch

dev-start-api: # Running the api on mode watch (Non container mode)
	RUST_LOG=INFO bacon api

dev-parquet: # Build in dev mode the MET CLI  (Non container mode)
	cargo build --bin parquet_population

dev-parquet-help: # Command to show the command need it to run the cli on dev mode
	@echo "cargo run --bin parquet_population"

build-prod-api-image: check_image_tag_env_var # Building a prod imag locally
	@echo "Building container image"
	docker build -f docker/server/Dockerfile -t afsoon/metered_usage_rust:${IMAGE_TAG} --target prod .

build-and-publish-prod-api-image: check_docker_env_var build-prod-api-image # Building a prod imag locally and publishing it in Github Container Registry
	@echo "Login to Github Container registry"
	echo $CR_PAT | docker login ghcr.io -u afsoon --password-stdin

	@echo "Publishing container image"
	docker push ghcr.io/afsoon/metered_usage_rust:${IMAGE_TAG}

start-again: # Nuke all images, volumes and unused images to rebuild from zero our compose.
	@echo "Shutting down containers"
	docker compose down --rmi all

	@echo "Pruning volumes"
	docker volume prune --force

	@echo "Removing unused images"
	docker image prune -a --force

	@echo "Rebuilding docker compose"
	docker compose build --no-cache

test-post: #Temporal make command until we have a cli tool to test it
	curl -X POST --header "Content-Type: application/json" -d @body_test_request.json http://localhost:8000
