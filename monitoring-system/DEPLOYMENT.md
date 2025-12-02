# Monitoring System Build and Deployment Guide

## Build Instructions

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies (Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y \
    pkg-config \
    libssl-dev \
    libsystemd-dev \
    libpcap-dev \
    build-essential

# For RHEL/CentOS
sudo yum install -y \
    pkg-config \
    openssl-devel \
    systemd-devel \
    libpcap-devel \
    gcc
```

### Building from Source

```bash
cd monitoring-system

# Build all components in release mode
cargo build --release --all

# Binaries will be in:
# - target/release/monitoring-agent
# - target/release/monitoring-collector
```

### Building with Specific Features

```bash
# Agent with all features
cd monitoring-agent
cargo build --release --features journald,pcap-capture,lz4-compression

# Agent without traffic capture (lighter build)
cargo build --release --features journald

# Collector with ClickHouse support
cd monitoring-collector
cargo build --release --features clickhouse-storage
```

## Installation

### Manual Installation

```bash
# Install binaries
sudo cp target/release/monitoring-agent /usr/local/bin/
sudo cp target/release/monitoring-collector /usr/local/bin/

# Create configuration directory
sudo mkdir -p /etc/monitoring

# Copy configuration files
sudo cp config/agent.toml /etc/monitoring/
sudo cp config/collector.toml /etc/monitoring/

# Create monitoring user
sudo useradd -r -s /bin/false monitoring
sudo useradd -r -s /bin/false monitoring-collector

# Install systemd services
sudo cp deployment/systemd/monitoring-agent.service /etc/systemd/system/
sudo cp deployment/systemd/monitoring-collector.service /etc/systemd/system/

# Set permissions
sudo chown -R monitoring:monitoring /etc/monitoring/agent.toml
sudo chown -R monitoring-collector:monitoring-collector /etc/monitoring/collector.toml

# Enable and start services
sudo systemctl daemon-reload
sudo systemctl enable monitoring-collector
sudo systemctl start monitoring-collector
sudo systemctl enable monitoring-agent
sudo systemctl start monitoring-agent
```

### Verify Installation

```bash
# Check agent status
sudo systemctl status monitoring-agent

# Check collector status
sudo systemctl status monitoring-collector

# View agent logs
sudo journalctl -u monitoring-agent -f

# Test agent configuration
monitoring-agent --config /etc/monitoring/agent.toml check

# Test connection to collector
monitoring-agent --config /etc/monitoring/agent.toml test-connection
```

## Docker Deployment

### Build Docker Images

```bash
cd monitoring-system

# Build agent image
docker build -f deployment/docker/Dockerfile.agent -t monitoring-agent:latest .

# Build collector image
docker build -f deployment/docker/Dockerfile.collector -t monitoring-collector:latest .
```

### Run with Docker

```bash
# Run collector
docker run -d \
  --name monitoring-collector \
  -p 8080:8080 \
  -e JWT_SECRET=your-secret \
  -v $(pwd)/config/collector.toml:/etc/monitoring/collector.toml:ro \
  monitoring-collector:latest

# Run agent
docker run -d \
  --name monitoring-agent \
  -v /var/log:/var/log:ro \
  -e MONITORING_AUTH_TOKEN=your-token \
  -v $(pwd)/config/agent.toml:/etc/monitoring/agent.toml:ro \
  monitoring-agent:latest
```

### Docker Compose

```bash
cd deployment
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

## Kubernetes Deployment

### Prerequisites

```bash
# Create namespace
kubectl create namespace monitoring

# Create secret for authentication
kubectl create secret generic monitoring-secrets \
  --from-literal=auth-token=your-auth-token \
  --from-literal=jwt-secret=your-jwt-secret \
  -n monitoring
```

### Deploy Collector

```bash
kubectl apply -f deployment/kubernetes/collector-deployment.yaml

# Verify deployment
kubectl get pods -n monitoring -l app=monitoring-collector

# Check logs
kubectl logs -n monitoring -l app=monitoring-collector -f
```

### Deploy Agent DaemonSet

```bash
kubectl apply -f deployment/kubernetes/daemonset.yaml

# Verify agent is running on all nodes
kubectl get pods -n monitoring -l app=monitoring-agent -o wide

# Check logs from specific node
kubectl logs -n monitoring monitoring-agent-xxxxx -f
```

### Access Collector

```bash
# Port-forward for local testing
kubectl port-forward -n monitoring svc/monitoring-collector 8080:8080

# Or expose via LoadBalancer/Ingress
kubectl patch svc monitoring-collector -n monitoring -p '{"spec": {"type": "LoadBalancer"}}'
```

## Configuration

### Agent Configuration

Edit `/etc/monitoring/agent.toml`:

```toml
[agent]
id = "unique-agent-id"  # Auto-generated if empty
hostname = ""           # Auto-detected if empty
tags = ["env:prod", "role:webserver"]

[collector]
endpoint = "ws://collector.example.com:8080/ingest"
auth_token = "${MONITORING_AUTH_TOKEN}"  # From environment

[collectors.logs]
enabled = true
files = [
    "/var/log/nginx/*.log",
    "/var/log/app/**/*.log"
]
journald_units = ["nginx.service", "app.service"]

[collectors.metrics]
enabled = true
system_interval_secs = 10

[collectors.traffic]
enabled = false  # Requires NET_ADMIN capability
```

### Collector Configuration

Edit `/etc/monitoring/collector.toml`:

```toml
[server]
websocket_addr = "0.0.0.0:8080"

[auth]
mode = "token"
token_secret = "${JWT_SECRET}"

[storage]
backend = "console"  # Change to clickhouse, postgres, or s3

[processor]
workers = 4
batch_size = 1000
```

## Troubleshooting

### Agent Issues

```bash
# Check if agent is running
sudo systemctl status monitoring-agent

# View recent logs
sudo journalctl -u monitoring-agent -n 100

# Test configuration
monitoring-agent --config /etc/monitoring/agent.toml check

# Test connection
monitoring-agent --config /etc/monitoring/agent.toml test-connection

# Run in foreground for debugging
monitoring-agent --config /etc/monitoring/agent.toml start --foreground
```

### Collector Issues

```bash
# Check collector status
sudo systemctl status monitoring-collector

# View logs
sudo journalctl -u monitoring-collector -n 100

# Test health endpoint
curl http://localhost:8080/health
```

### Common Issues

**Permission Denied on Log Files:**
```bash
# Add monitoring user to adm group
sudo usermod -a -G adm monitoring
sudo systemctl restart monitoring-agent
```

**Packet Capture Not Working:**
```bash
# Grant cap_net_raw capability
sudo setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/monitoring-agent
```

**Connection Refused:**
```bash
# Check collector is running
curl http://collector-host:8080/health

# Check firewall
sudo ufw allow 8080/tcp
```

## Performance Tuning

### Agent

```toml
[buffer]
max_events = 20000      # Increase for high-volume systems
flush_interval_secs = 30 # Decrease for lower latency

[collectors.metrics]
system_interval_secs = 30 # Increase to reduce overhead
```

### Collector

```toml
[processor]
workers = 8  # Increase for more parallel processing
```

## Security Best Practices

1. **Use TLS/mTLS in production:**
   ```toml
   [collector]
   tls_ca_cert = "/etc/monitoring/ca.pem"
   client_cert = "/etc/monitoring/client.pem"
   client_key = "/etc/monitoring/client-key.pem"
   ```

2. **Rotate JWT secrets regularly**

3. **Run as non-root user** (already configured)

4. **Use strong authentication tokens**

5. **Enable audit logging**

## Monitoring the Monitor

- Expose Prometheus metrics from the collector
- Monitor agent resource usage
- Alert on failed batch deliveries
- Track ingestion rate and latency

## Backup and Recovery

### Configuration Backup

```bash
# Backup configurations
sudo tar -czf monitoring-config-backup.tar.gz /etc/monitoring/

# Restore
sudo tar -xzf monitoring-config-backup.tar.gz -C /
```

### Data Recovery

Depends on storage backend (ClickHouse, S3, etc.)
