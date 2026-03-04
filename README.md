# caduceus-term

Terminal proxy library with WASM-based I/O stream transformation.

Caduceus wraps a child process and proxies its stdin, stdout, and stderr through optional WebAssembly transform modules. Each stream can have its own independent WASM transform, allowing you to filter, modify, or inspect terminal I/O in real time.

## Features

- **Piped and PTY child modes** — spawn child processes with standard pipes or a pseudo-terminal
- **Per-stream WASM transforms** — attach independent WASM modules to stdin, stdout, and/or stderr
- **Async host functions** — WASM guest modules can call back into the host to read stdin or write stdout/stderr
- **Serialized I/O queue** — all physical I/O is funneled through a single async queue, preventing interleaving
- **Builder API** — fluent `ProxyBuilder` for configuration

## WASM Guest Contract

Transform modules must export:

| Export | Signature | Description |
|---|---|---|
| `memory` | Memory | Linear memory for data exchange |
| `alloc` | `(i32) -> i32` | Allocate `n` bytes, return pointer (0 = failure) |
| `transform` | `(i32, i32) -> i64` | Transform bytes at `(ptr, len)`. Returns packed `(out_ptr << 32) \| out_len` |

Optional host imports (in the `"env"` namespace):

| Import | Signature | Description |
|---|---|---|
| `host_read_stdin` | `(i32, i32) -> i32` | Read up to `max_len` bytes into `buf_ptr`. Returns bytes read. |
| `host_write_stdout` | `(i32, i32)` | Write `len` bytes from `ptr` to parent stdout |
| `host_write_stderr` | `(i32, i32)` | Write `len` bytes from `ptr` to parent stderr |

## Usage

```rust
use caduceus_term::proxy::run_proxy;
use caduceus_term::{ChildMode, ProxyBuilder, WasmModuleSource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = std::fs::read("my_transform.wasm")?;

    let config = ProxyBuilder::new("bash")
        .arg("-i")
        .child_mode(ChildMode::Piped)
        .stdout_transform(WasmModuleSource::Bytes(wasm_bytes))
        .build();

    let status = run_proxy(config).await?;
    std::process::exit(status.code().unwrap_or(1));
}
```

## Feature Flags

| Flag | Default | Description |
|---|---|---|
| `pty` | yes | PTY child mode via `pty-process` |

Build without PTY support:

```sh
cargo build --no-default-features
```

## License

MIT
