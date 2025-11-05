.PHONY: help dev build test clean docker-up docker-down migrate db-reset

help: ## Show this help message
	@echo "CoreCRM-R Development Commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

dev: ## Start development environment (PostgreSQL + all services)
	docker-compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	cargo run --package metadata-service

dev-metadata: ## Start only metadata service
	cargo run --package metadata-service

build: ## Build all services
	cargo build --workspace

build-release: ## Build all services in release mode
	cargo build --workspace --release

test: ## Run all tests
	cargo test --workspace

test-verbose: ## Run tests with verbose output
	cargo test --workspace -- --nocapture

clean: ## Clean build artifacts
	cargo clean
	docker-compose down -v

docker-up: ## Start all services with Docker Compose
	docker-compose up -d

docker-down: ## Stop all Docker services
	docker-compose down

docker-logs: ## Show logs from Docker services
	docker-compose logs -f

docker-build: ## Build Docker images
	docker-compose build

migrate: ## Run database migrations
	cd services/metadata-service && sqlx migrate run

migrate-revert: ## Revert last migration
	cd services/metadata-service && sqlx migrate revert

db-reset: ## Reset database (drop and recreate)
	docker-compose down -v
	docker-compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	cd services/metadata-service && sqlx migrate run

check: ## Check code without building
	cargo check --workspace

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

clippy: ## Run clippy lints
	cargo clippy --workspace -- -D warnings

watch: ## Watch and rebuild on changes (requires cargo-watch)
	cargo watch -x 'run --package metadata-service'

install-tools: ## Install development tools
	cargo install sqlx-cli --no-default-features --features postgres
	cargo install cargo-watch

db-shell: ## Open PostgreSQL shell
	docker exec -it corecrm-postgres psql -U corecrm -d corecrm_metadata
