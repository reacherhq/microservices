[![Actions Status](https://github.com/reacherhq/backend/workflows/pr/badge.svg)](https://github.com/reacherhq/backend/actions)
![GitHub](https://img.shields.io/github/license/reacherhq/backend.svg)

# Reacher Backend

This repo holds the backend for [Reacher](https://reacherhq.github.io/). The backend is a HTTP server around the Rust library [`check-if-email-exists`](https://github.com/amaurymartiny/check-if-email-exists), which performs the core email verification logic.

## Documentation: https://reacher.email/docs

## Get Started

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/reacherhq/backend)

To run the server, just run:

```bash
cargo run
```

The server will then be listening on `http://127.0.0.1:8080`.

These are the environment variables used to configure the HTTP server:

| Env Var              | Required? | Description                                                                        |
| -------------------- | --------- | ---------------------------------------------------------------------------------- |
| `RCH_FROM_EMAIL`     | No        | The email to use in the `MAIL FROM:` SMTP command.                                 |
| `RCH_PROXY_HOST`     | No        | Use the specified SOCKS5 proxy host to perform email verification.                 |
| `RCH_PROXY_PORT`     | No        | Use the specified SOCKS5 proxy port to perform email verification.                 |
| `RCH_SAASIFY_SECRET` | No        | If set, all incoming requests will need to have a `x-saasify-proxy-secret` header. |
| `RCH_SENTRY_DSN`     | No        | [Sentry](https://sentry.io) DSN used for bug reports.                              |

## See also

-   [`check-if-email-exists`](https://github.com/amaurymartiny/check-if-email-exists): Rust library to check if an email address exists without sending any email.
