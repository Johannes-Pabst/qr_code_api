# QR Code API (Rust + Actix)

A lightweight HTTP API for generating QR codes from URLs or text.  
Built with **Rust**, **Actix Web**, and the **qrcode** crate.

The service generates QR codes in multiple formats depending on flags embedded in the request URL.

Supported output formats:

- PNG
- JPG
- SVG
- Terminal text (UTF-8)

It also supports different **QR error correction levels** and **download behavior**.

---

# Running the Server
 ## A demo server is running on qr.sach.si.
Build and run the server:

```bash
cargo run --release
```

The server will start on:

http://localhost:8081

---

# Basic Usage

The API works by placing **flags** and the **target content** directly in the request path.

General format:

```
http://HOST:PORT/<flags>/<target>
```

Example:

```
http://localhost:8081/png/https://example.com
```

---

# Default Behavior

If no flags are provided:

- Format: `PNG`
- Error correction: default
- Display: browser inline

Example:

```
http://localhost:8081/https://example.com
```

---

# Output Format Flags

You can select the output format with these flags:

| Flag | Output |
|-----|------|
| `png` | PNG image |
| `jpg` | JPEG image |
| `svg` | SVG image |
| `text` | Terminal-friendly text QR |

Example paths:

```
/png/https://example.com
/jpg/https://example.com
/svg/https://example.com
/text/https://example.com
```

Example request:

```
http://localhost:8081/svg/https://example.com
```

---

# Error Correction Levels

QR codes support four levels of error correction.

| Flag | Level | Recovery |
|-----|------|------|
| `low` | L | ~7% |
| `medium` | M | ~15% |
| `quartile` | Q | ~25% |
| `high` | H | ~30% |

Example:

```
http://localhost:8081/png-high/https://example.com
```

---

# Download vs Browser Display

Control how the browser handles the generated file.

| Flag | Behavior |
|----|----|
| `browser` | Display inline (default) |
| `download` | Force file download |

Example:

```
http://localhost:8081/png-download/https://example.com
```

This will download:

```
qrcode.png
```

---

# Combining Flags

Flags can be combined using `-`.

Example:

```
/png-high-download/
```

Full example:

```
http://localhost:8081/png-high-download/https://example.com
```

This will:

- generate a **PNG**
- with **high error correction**
- and **force download**

---

# Text Mode (Terminal QR)

If the request is made using `curl`, the API automatically returns a terminal-friendly QR code.

Example:

```bash
curl http://localhost:8081/https://example.com
```

Example output:

```
 ▄▄▄▄▄ █ ▄▄▄▄█▄██▀▄▀██▄███ ▄▄▄▄▄ 
 █   █ █▄▄█████▄▄▄▄▀▄ ▀█▀█ █   █ 
 █▄▄▄█ █▄██▀██▄▄█▀ █  ▀▀▀█ █▄▄▄█ 
▄▄▄▄▄▄▄█▄█ ▀ ▀▄█▄█ ▀ █▄█ █▄▄▄▄▄▄▄
 ██▄  ▄▄▄▄▀▀██ ▄█▄▀ █▀ ▀█▄██ ▀ ▄▄
 █ ▀ ▄▄▀▄▀█▄ █ ▄▄ ▄▀▄█▄   ▀▀▄▄ █▄
█ ▀ ▀█▄▄▀█▄ █ ▄█ ▄▄▀▄ ▄▀█▀▄ █▀██▄
▄▀█ █▄▄▀▄ ▀██▄▄▀▄█▄▀██▀█▀▀▄█▀ ▀█▀
▀ ▄▄▀█▄██▄  ███▀▀█▀▄ ▀ ▄▄▄ ▄ █▀▀▀
▄ ▄  ▄▄▀▀█▄▄▀   █▀▀▄█▀█▄▄▀ ▄▀  █▄
▀▄██▄ ▄█▀▄█  ▀▄▄▀▀▀█ ▄▀▀▄██ ▀▄▀▄█
 █▄█ █▄▀█▀▄▄█▄▄▄ ▀▀████▄█  ▀▄█ ▄ 
▄▄▄▄▄█▄▄  ▀▄▄▄▄ ▀█▀▄ ██▀ ▄▄▄ █▀██
 ▄▄▄▄▄ █ ██▄ █ ▄██▄▄▄▄▄  █▄█ ▀ ██
 █   █ █ ██▀▀██▄█▄▄▄  █▄  ▄ ▄▀▀█ 
 █▄▄▄█ ██▄▄ ▀ ▄▀ █▀█▄█▀ ▀▄▀   ▀  
       ▀   ▀▀▀▀▀  ▀▀▀▀▀  ▀▀    ▀▀

```

You can also explicitly request text mode:

```
http://localhost:8081/text/https://example.com
```

---

# SVG Behavior

SVG output wraps the QR code in a clickable link pointing to the encoded target.

This allows the QR code itself to act as a hyperlink in browsers.

---

# Error Handling

Invalid flags return:

```
400 Bad Request
Invalid format or flag provided.
```

If the QR content is too large:

```
413 Payload Too Large
Failed to generate QR code, probably too long URL or invalid characters.
```

---

# Example Requests

### PNG QR code

```
http://localhost:8081/png/https://google.com
```

### SVG QR code

```
http://localhost:8081/svg/https://github.com
```

### High correction JPEG

```
http://localhost:8081/jpg-high/https://example.com
```

### Download PNG

```
http://localhost:8081/png-download/https://example.com
```

### Terminal QR code

```bash
curl http://localhost:8081/text/HelloWorld
```

---

# Dependencies

Key crates used:

- `actix-web`
- `qrcode`
- `image`
