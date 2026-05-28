# jwtk

A minimal CLI for encoding and decoding JWTs signed with HMAC-SHA256.

## Install

```bash
cargo install --path .
```

## Usage

### Encode

```bash
jwtk encode --secret <SECRET> --payload '<JSON>'
```

**Example:**

```bash
jwtk encode --secret mysecret --payload '{"sub":"1234","name":"Alice","iat":1716000000}'
# Generated JWT token: eyJ0eXAi...
```

### Decode

Verify and decode (signature checked):

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
 Algorithm.................: Some(HS256)

Payload:
 iat.......................: 1716000000
 name......................: "Alice"
 sub.......................: "1234"
```

## Build

```bash
cargo build --release
```

## License

MIT
