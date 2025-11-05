.PHONY: help dev build test clean docker-up docker-down migrate db-reset

help: ## Show this help message
	@echo "CoreCRM-R Development Commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

dev: ## Start development environment (PostgreSQL + all services)
	docker-compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 3
	@echo "Starting services in parallel..."
	@echo "Metadata Service: http://localhost:8001"
	@echo "API Gateway: http://localhost:8000"
	@make -j2 dev-metadata dev-gateway

dev-metadata: ## Start only metadata service
	cargo run --package metadata-service

dev-gateway: ## Start only API gateway
	cargo run --package api-gateway

dev-docker: ## Start all services with Docker Compose
	docker-compose up -d
	@echo "Services started:"
	@echo "  Metadata Service: http://localhost:8001"
	@echo "  API Gateway: http://localhost:8000"
	@echo "  PostgreSQL: localhost:5432"

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

migrate: ## Run database migrations for all services
	cd services/metadata-service && sqlx migrate run
	cd services/api-gateway && sqlx migrate run

migrate-metadata: ## Run metadata service migrations only
	cd services/metadata-service && sqlx migrate run

migrate-gateway: ## Run API gateway migrations only
	cd services/api-gateway && sqlx migrate run

migrate-revert: ## Revert last migration (metadata service)
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

db-shell-metadata: ## Open PostgreSQL shell (metadata DB)
	docker exec -it corecrm-postgres psql -U corecrm -d corecrm_metadata

db-shell-gateway: ## Open PostgreSQL shell (gateway DB)
	docker exec -it corecrm-postgres psql -U corecrm -d corecrm_gateway

db-shell: db-shell-metadata ## Alias for db-shell-metadata

login: ## Login to API Gateway (default admin user)
	@curl -X POST http://localhost:8000/api/v1/auth/login \
		-H "Content-Type: application/json" \
		-d '{"username": "admin", "password": "admin123"}' | jq

demo: ## Run MVP 2 demo script
	@bash scripts/demo-mvp2.sh
