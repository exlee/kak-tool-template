# kak-tool-template

A blazing fast, async Rust template for building external Kakoune tools.

This template provides a robust architecture for creating "fire-and-forget" binaries that interact with Kakoune. It uses a self-spawning fork model to ensure the editor interface never freezes, achieving runtimes as low as **0.008s** on the main thread.

## Philosophy

- **Zero Blocking**: The main process spawns a background worker and exits immediately. Kakoune continues processing user input instantly.

- **Single Binary**: No separate daemons to manage. The binary acts as both the launcher and the worker.

- **Async I/O**: Powered by Tokio for high-performance file and socket operations.

- **Robust FIFO Handling**: Includes boilerplate for correctly synchronizing FIFO streams with Kakoune's edit -fifo command, preventing deadlocks and "split-brain" read issues.

## Architecture

1. **Launcher Mode**: When called by Kakoune, the binary parses arguments, spawns a detached copy of itself with the --worker flag, and exits.

2. **Worker Mode**: The detached process initializes the Tokio runtime, connects to Kakoune's socket (via kak-p logic), and executes heavy tasks (search, processing, I/O).

## Getting Started

1. Clone the repository:

```
git clone [https://github.com/exlee/kak-tool-template](https://github.com/exlee/kak-tool-template)
cd kak-tool-template
```

2. Modify `src/main.rs`: Implement your logic in `handle_command`. The default example demonstrates writing to a temporary FIFO buffer.

3. Build:

```
cargo build --release
```

4. Integration:
Add the binary to your path and define a command in your kakrc:

```kakoune
define-command rust-example %{
    %sh{
        # Format: binary <session> <client> [args...]
        /path/to/target/release/kak-tool "$kak_session" "$kak_client" "some-arg"
    }
}
```


## Example Usage

The provided template implements a simple FIFO interaction:

1. Rust creates a named pipe in /tmp.
2. Rust commands Kakoune to open this pipe (edit! -fifo ...).
3. Rust writes "Hello World" to the pipe asynchronously.
4. Rust closes the pipe, triggering EOF in Kakoune.
5. Rust sends a "PONG" message to the client's echo area.

## Templating

This repository includes a configuration file for [tmplr](https://github.com/exlee/tmplr) for convenience.

## License

MIT
