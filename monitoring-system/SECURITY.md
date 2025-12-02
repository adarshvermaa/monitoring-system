# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@example.com**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

Please include the following information:
- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Process

1. **Acknowledgment**: We'll acknowledge receipt within 48 hours
2. **Investigation**: We'll investigate and provide an initial assessment within 7 days
3. **Fix**: We'll develop and test a fix
4. **Disclosure**: We'll coordinate disclosure timing with you
5. **Credit**: You'll be credited in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

### For Deployment

1. **Use TLS/mTLS**: Always use encrypted connections in production
   ```toml
   [collector]
   tls_ca_cert = "/etc/monitoring/ca.pem"
   client_cert = "/etc/monitoring/client.pem"
   client_key = "/etc/monitoring/client-key.pem"
   ```

2. **Rotate Credentials**: Regularly rotate JWT secrets and auth tokens
   ```bash
   # Generate new secret
   openssl rand -base64 32
   ```

3. **Least Privilege**: Run services with minimal permissions
   ```bash
   # Already configured in systemd units
   User=monitoring
   NoNewPrivileges=true
   ```

4. **Network Isolation**: Use firewalls and network policies
   ```bash
   # Only allow collector port
   sudo ufw allow 8080/tcp
   ```

5. **Update Dependencies**: Keep Rust and dependencies updated
   ```bash
   cargo update
   cargo audit
   ```

### Known Security Considerations

1. **Packet Capture**: Requires CAP_NET_RAW capability - use with caution
2. **Log Files**: May contain sensitive data - ensure proper access controls
3. **Metrics**: May reveal system information - restrict access to collector
4. **WebSocket**: No rate limiting by default - add reverse proxy if needed

## Security Audits

This project has not yet undergone an external security audit. 

If you're interested in sponsoring a security audit, please contact us.

## Updates

We'll use GitHub Security Advisories to announce security updates:
- https://github.com/yourorg/monitoring-system/security/advisories

## Contact

- Security issues: security@example.com
- General questions: dev@example.com
- Discord: discord.gg/monitoring
