# Repository Guidelines
- Rust toolchain: 1.90.0
- Build system: Cargo (standard workspace).
- Module layout: prefer `foo.rs` alongside `foo/` submodules instead of `foo/mod.rs`.
- Network fetch components should use async Rust APIs.
- Match the formatting illustrated in `sample/` artifacts for config, state, logs, reports, and launchd templates.
