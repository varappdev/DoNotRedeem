# Extension

Chrome / Firefox (Manifest V3) popup extension, built with Dioxus for the web.
Shared components come from the [`ui`](../ui) crate.

## Build

```sh
cd extension
dx bundle --platform web --release
```

The unpacked extension is written to `target/dx/extension/release/web/public/`
(at the workspace root). `public/manifest.json` is copied into it as-is.

## Load

- **Chrome:** `chrome://extensions` → enable *Developer mode* → *Load unpacked* → pick the folder above.
- **Firefox:** `about:debugging#/runtime/this-firefox` → *Load Temporary Add-on…* → pick `manifest.json` in the folder above.

The manifest's CSP allows `'wasm-unsafe-eval'`, which both browsers require to instantiate WebAssembly on extension pages.
