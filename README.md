# jwtk

A minimal CLI for encoding and decoding JWTs.

## Install

```bash
cargo install --path .
```

## Usage

### Encode

```bash
jwtk encode --secret <SECRET> --payload '<JSON>' [--expire <SECONDS>] [--alg <ALGORITHM>]
```

`--expire` sets the token lifetime in seconds from now. Defaults to `3600` (1 hour). The `exp` claim is added automatically.

`--alg` sets the signing algorithm. Defaults to `HS256`. Supported values:

| Family | Algorithms |
|--------|------------|
| HMAC | `HS256`, `HS384`, `HS512` |
| RSA PKCS#1 | `RS256`, `RS384`, `RS512` |
| RSA-PSS | `PS256`, `PS384`, `PS512` |
| ECDSA | `ES256`, `ES384` |
| EdDSA | `EdDSA` |

If `--secret` is given without a value (or with `-`), the secret is read interactively from stdin.

**Examples:**

```bash
# Secret as argument
jwtk encode --secret mysecret --payload '{"sub":"1234","name":"Alice"}' --expire 300

# Secret read from stdin (prompted)
jwtk encode --secret --payload '{"sub":"1234","name":"Alice"}'

# Secret piped from another command
echo -n mysecret | jwtk encode --secret - --payload '{"sub":"1234","name":"Alice"}'
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

If `--token` or `--secret` is given without a value (or with `-`), it is read from stdin.

**Stdin examples:**

```bash
# Token piped in, secret as argument
echo '<TOKEN>' | jwtk decode --token - --secret mysecret

# Both token and secret read interactively
jwtk decode --token --secret

# Token piped, no signature verification
echo '<TOKEN>' | jwtk decode --token -
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

## Installation

### Install script (macOS & Linux)

The quickest way to get `jwtk` is the install script, which detects your OS
(macOS or Linux) and architecture (Apple Silicon / ARM64 or Intel / x86_64),
downloads the latest prebuilt binary, and installs it to `/usr/local/bin`:

```bash
curl -fsSL https://raw.githubusercontent.com/kgantsov/jwtk/main/install.sh | bash
```

Or download and run it manually if you'd rather inspect it first:

```bash
curl -fsSLO https://raw.githubusercontent.com/kgantsov/jwtk/main/install.sh
bash install.sh
```

The script may prompt for `sudo` to write to `/usr/local/bin`. Once it finishes,
run `jwtk --help` to verify.

### Build

```bash
cargo build --release
```

## License

MIT
