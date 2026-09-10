# android-sys-compat

Drop-in replacements for Linux `-sys` crates that don't build on Android.

## Why

`pkg-config` fails when cross-compiling to Android because the native libraries
aren't in the NDK. Each crate here replaces one of them.

## How

- No `pkg-config` at build time.
- Library loaded at runtime via `dlopen` (if present).
- Missing library → returns null/zero, consumers see "no devices".

## Crates

- `libudev-sys`
- `drm-sys`
- `libinput-sys`
- `libseat-sys`
- `gbm-sys`
- `and more...`

## Usage

```toml
[patch.crates-io]
libudev-sys = { git = "https://github.com/YOUR_USERNAME/android-sys-compat", branch = "main" }
