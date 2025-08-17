# TLS/SSL Configuration

Configure secure connections to your MySQL/MariaDB database.

## TLS Support

Gold Digger supports TLS/SSL through:

- Platform-native TLS (default)
- Pure Rust TLS implementation (optional)

## Connection String Parameters

Configure TLS in your connection URL:

```
mysql://user:pass@host:3306/db?ssl-mode=required
```

## SSL Modes

| Mode              | Description                       |
| ----------------- | --------------------------------- |
| `disabled`        | No SSL connection                 |
| `preferred`       | SSL if available, plain otherwise |
| `required`        | Require SSL connection            |
| `verify-ca`       | Require SSL and verify CA         |
| `verify-identity` | Require SSL and verify identity   |

## Certificate Validation

For production environments, always use `verify-ca` or `verify-identity` modes to prevent man-in-the-middle attacks.

## Troubleshooting TLS

Common TLS issues and solutions:

- Certificate verification failures
- Protocol version mismatches
- Cipher suite compatibility
