#

## Installation

Step by step to create this project

#### create new rust project

```bash
cargo new zen-disc-backend --bin
```

#### add in `Cargo.toml`

```rust
[dependencies]
diesel = { version = "1.4", features = ["postgres"] }
```

#### create database

```bash
createdb db_zen_disc
```

#### run

```bash
cargo watch -x 'run'
```
