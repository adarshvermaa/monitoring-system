# Quick Reference Guide

## Common Commands

### Build & Test
```bash
make build          # Build release
make test           # Run tests
make check          # Lint + format + test
make clean          # Clean artifacts
```

### Run Locally
```bash
make run-local      # Start both collector and agent
# OR manually:
make run-collector  # Terminal 1
make run-agent      # Terminal 2
```

### Docker
```bash
make docker-build   # Build images
make docker-run     # Run with compose
make docker-logs    # View logs
make docker-down    # Stop containers
```

### Kubernetes
```bash
make k8s-deploy     # Deploy to cluster
make k8s-logs       # View logs
make k8s-delete     # Remove deployment
```

## Configuration Quick Reference

### Agent Config (`config/agent.toml`)
```toml
[agent]
id = "my-agent"
tags = ["env:prod"]

[collector]
endpoint = "ws://collector:8080/ingest"
auth_token = "${TOKEN}"

[collectors.logs]
enabled = true
files = ["/var/log/**/*.log"]

[collectors.metrics]
enabled = true
system_interval_secs = 10

[collectors.traffic]
enabled = false  # Requires NET_ADMIN
```

### Collector Config (`config/collector.toml`)
```toml
[server]
websocket_addr = "0.0.0.0:8080"

[auth]
mode = "token"
token_secret = "${JWT_SECRET}"

[storage]
backend = "console"  # or clickhouse/postgres/s3
```

## Environment Variables

```bash
# Agent
export MONITORING_AUTH_TOKEN="your-token"

# Collector
export JWT_SECRET="your-secret"
export RUST_LOG="info"  # debug, info, warn, error
```

## Health Checks

```bash
# Collector health
curl http://localhost:8080/health

# Agent config check
monitoring-agent --config config/agent.toml check

# Test connection
monitoring-agent --config config/agent.toml test-connection
```

## Logs

```bash
# Systemd
sudo journalctl -u monitoring-agent -f
sudo journalctl -u monitoring-collector -f

# Docker
docker logs monitoring-agent -f
docker logs monitoring-collector -f

# Kubernetes
kubectl logs -n monitoring -l app=monitoring-agent -f
kubectl logs -n monitoring -l app=monitoring-collector -f

# Local files
tail -f logs/agent.log
tail -f logs/collector.log
```

## Troubleshooting

### Agent won't start
```bash
# Check config
monitoring-agent --config config/agent.toml check

# Check permissions
ls -la /var/log/  # Must be readable

# Run in foreground
monitoring-agent --config config/agent.toml start --foreground
```

### Connection refused
```bash
# Check collector is running
curl http://localhost:8080/health

# Check firewall
sudo ufw status
sudo ufw allow 8080/tcp

# Check network
ping collector-host
telnet collector-host 8080
```

### High CPU usage
```toml
# Reduce collection frequency
[collectors.metrics]
system_interval_secs = 30  # Instead of 10

# Disable traffic capture
[collectors.traffic]
enabled = false
```

### Memory issues
```toml
# Reduce buffer size
[buffer]
max_events = 5000  # Instead of 10000
flush_interval_secs = 30  # Flush more often
```

## Performance Tuning

### Agent
- Increase `flush_interval_secs` to reduce network calls
- Decrease `system_interval_secs` to reduce CPU
- Use `compression = "snappy"` for best speed/ratio balance
- Disable unused collectors

### Collector
- Increase `workers` for more parallel processing
- Increase `batch_size` for better throughput
- Use appropriate storage backend for your scale

## Security Checklist

- [ ] Use TLS/mTLS in production
- [ ] Rotate JWT secrets monthly
- [ ] Run as non-root user
- [ ] Restrict collector port with firewall
- [ ] Use strong auth tokens (32+ chars)
- [ ] Enable audit logging
- [ ] Keep dependencies updated (`cargo update`)
- [ ] Review access to log files

## Useful Links

- [README](README.md) - Full documentation
- [DEPLOYMENT](DEPLOYMENT.md) - Deployment guide
- [CONTRIBUTING](CONTRIBUTING.md) - Contribution guidelines
- [SECURITY](SECURITY.md) - Security policy
