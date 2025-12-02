# Monitoring System Makefile

.PHONY: help build test clean run-collector run-agent docker-build docker-run install lint fmt check-fmt doc

# Default target
.DEFAULT_GOAL := help

help: ## Show this help message
	@echo "Monitoring System - Available Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build all components in release mode
	cargo build --release --all

build-dev: ## Build all components in debug mode
	cargo build --all

test: ## Run all tests
	cargo test --all

test-verbose: ## Run tests with verbose output
	cargo test --all -- --nocapture

clean: ## Clean build artifacts
	cargo clean
	rm -rf logs/

run-collector: ## Run the collector server
	@echo "Starting collector on ws://localhost:8080..."
	JWT_SECRET=dev-secret cargo run -p monitoring-collector -- --config config/collector.toml

run-agent: ## Run the monitoring agent
	@echo "Starting agent..."
	MONITORING_AUTH_TOKEN=dev-token cargo run -p monitoring-agent -- --config config/agent.toml

run-local: build ## Build and run both collector and agent
	@./scripts/start-local.sh

docker-build: ## Build Docker images
	docker build -f deployment/docker/Dockerfile.agent -t monitoring-agent:latest .
	docker build -f deployment/docker/Dockerfile.collector -t monitoring-collector:latest .

docker-run: ## Run with Docker Compose
	cd deployment && docker-compose up -d

docker-logs: ## View Docker Compose logs
	cd deployment && docker-compose logs -f

docker-down: ## Stop Docker Compose
	cd deployment && docker-compose down

install-deps: ## Install required system dependencies (Debian/Ubuntu)
	sudo apt-get update
	sudo apt-get install -y pkg-config libssl-dev libsystemd-dev libpcap-dev

install: build ## Install binaries to /usr/local/bin
	sudo cp target/release/monitoring-agent /usr/local/bin/
	sudo cp target/release/monitoring-collector /usr/local/bin/
	sudo mkdir -p /etc/monitoring
	sudo cp config/agent.toml /etc/monitoring/
	sudo cp config/collector.toml /etc/monitoring/
	@echo "✓ Installed successfully"

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code with rustfmt
	cargo fmt --all

check-fmt: ## Check code formatting
	cargo fmt --all -- --check

doc: ## Generate and open documentation
	cargo doc --all --no-deps --open

benchmark: ## Run benchmarks
	cargo bench --all

check: lint check-fmt test ## Run all checks (lint, format, test)

release: check build ## Build release after running all checks
	@echo "✓ Release build complete"

deploy-systemd: install ## Deploy with systemd
	sudo cp deployment/systemd/monitoring-agent.service /etc/systemd/system/
	sudo cp deployment/systemd/monitoring-collector.service /etc/systemd/system/
	sudo systemctl daemon-reload
	@echo "✓ Systemd services installed. Enable with:"
	@echo "  sudo systemctl enable --now monitoring-collector"
	@echo "  sudo systemctl enable --now monitoring-agent"

k8s-deploy: ## Deploy to Kubernetes
	kubectl create namespace monitoring --dry-run=client -o yaml | kubectl apply -f -
	kubectl apply -f deployment/kubernetes/collector-deployment.yaml
	kubectl apply -f deployment/kubernetes/daemonset.yaml

k8s-logs: ## View Kubernetes logs
	kubectl logs -n monitoring -l app=monitoring-collector -f

k8s-delete: ## Delete Kubernetes deployment
	kubectl delete -f deployment/kubernetes/daemonset.yaml
	kubectl delete -f deployment/kubernetes/collector-deployment.yaml

coverage: ## Generate code coverage report
	cargo tarpaulin --all --out Html --output-dir target/coverage

watch: ## Watch for changes and rebuild
	cargo watch -x 'build --all'

watch-test: ## Watch for changes and run tests
	cargo watch -x 'test --all'
