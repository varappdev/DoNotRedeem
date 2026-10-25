# Scamvoidance

A web extension aimed at combatting scam emails

The email is first fed through Jev to detect if it is a scam email, where afterwards it is fed through decision models to produce a human-readable output

## Quick Start

```sh
cd extension
OPENROUTER_API_KEY=... dx bundle --platform web --release
```

The unpacked extension is written to `target/dx/extension/release/web/public/`
(at the workspace root). `public/manifest.json` is copied into it as-is.

- **Add to Chrome:** `chrome://extensions` → enable *Developer mode* → *Load unpacked* → pick the folder above.
-
