# Security Best Practices

Comprehensive security guidelines for using Gold Digger in production.

## Production Deployment

### File Permissions

- Set restrictive permissions on output files containing sensitive data
- Use umask to control default file permissions
- Consider encrypting output files for highly sensitive data

### Credential Management

- Use environment variables instead of CLI flags for credentials
- Implement credential rotation procedures
- Use secrets management systems in containerized environments

### Network Security

- Always use TLS/SSL for database connections
- Restrict database access by IP address
- Use private networks or VPNs for sensitive operations

## Data Handling

### Output Security

- Review output files for sensitive information before sharing
- Implement data retention policies
- Use secure file transfer methods

### Query Safety

- Validate SQL queries before execution
- Use parameterized queries when possible
- Avoid exposing sensitive data in query logs

## Monitoring and Auditing

### Access Logging

- Log all database access attempts
- Monitor for unusual query patterns
- Implement alerting for security events

### Regular Security Reviews

- Audit database permissions regularly
- Review and update security configurations
- Test backup and recovery procedures

## Development Security

### Security Scanning

Gold Digger includes comprehensive security scanning tools for development and CI/CD:

```bash
# Run security audit for known vulnerabilities
just audit

# Check licenses and security policies
just deny

# Comprehensive security scan (audit + deny + grype)
just security

# Generate Software Bill of Materials (SBOM)
just sbom
```

### Dependency Management

- Regularly update dependencies to patch security vulnerabilities
- Use `cargo audit` to check for known security issues
- Review dependency licenses with `cargo deny`
- Generate and review SBOMs for supply chain security

### Vulnerability Scanning

The `just security` command performs comprehensive vulnerability scanning:

1. **Security Audit**: Uses `cargo audit` to check for known vulnerabilities in dependencies
2. **License Compliance**: Uses `cargo deny` to enforce license and security policies
3. **Container Scanning**: Uses `grype` to scan for vulnerabilities in the final binary

### Supply Chain Security

- All release artifacts are signed with Cosign using keyless OIDC
- SBOMs are generated for all releases in CycloneDX format
- Dependencies are tracked and audited automatically
- Use `just sbom` to inspect the software bill of materials locally
