# Aegis

**Aegis** is a modular, cross-platform post-exploitation framework written in Rust.  
It is designed to help operators interact with compromised systems in a structured, extensible, and scriptable way.

## Features

- **Session Management:** Manage multiple TCP/UDP sessions with remote hosts.
- **Modular Architecture:** Easily extend functionality by adding new modules for Linux and Windows.
- **Built-in Modules:**  
  - Download text files from Linux and Windows targets
  - Run system information commands (`whoami`, `uname`, `systeminfo`)
- **CLI Interface:** Interactive shell with command history and tab completion.
- **Scriptable:** Run scripts of commands on sessions for automation.

## Example Usage

```
run-module linux/download 1 /etc/passwd
run-module windows/systeminfo 2
```

## Project Structure

- `src/core/modules/` — All modules (Linux and Windows)
- `src/cli/` — Command-line interface and REPL logic
- `src/core/comms/` — Communication layer (TCP/UDP)
- `src/core/session.rs` — Session management

## Status

**This project is a work in progress.**  
Expect breaking changes, incomplete features, and evolving APIs.  
Contributions and feedback are welcome!