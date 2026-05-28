# jwtk

A minimal CLI for encoding and decoding JWTs signed with HMAC-SHA256.

## Install

```bash
cargo install --path .
```

## Usage

### Encode

```bash
jwtk encode --secret <SECRET> --payload '<JSON>' [--expire <SECONDS>]
```

`--expire` sets the token lifetime in seconds from now. Defaults to `3600` (1 hour). The `exp` claim is added automatically.

**Example:**

```bash
jwtk encode --secret mysecret --payload '{"sub":"1234","name":"Alice"}' --expire 300
# Generated JWT token: eyJ0eXAi...
```

### Decode

Verify and decode (signature and expiry checked):

```bash
jwtk decode --token <TOKEN> --secret <SECRET>
```

Decode without verification:

```bash
jwtk decode --token <TOKEN>
```

**Example output:**

```
Headers:
 Validated.................: true
 Algorithm.................: HS256

Expiration (exp)..........: 2026-05-28 13:00:00 (295 seconds remaining)

Payload:
 exp.......................: 1748433600
 name......................: "Alice"
 sub.......................: "1234"
```

## Build

```bash
cargo build --release
```

## License

MIT
