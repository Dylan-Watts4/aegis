# Aegis

**Aegis** is a modular, cross-platform post-exploitation and operator framework written in Rust.  
It empowers red teamers, penetration testers, and operators to interact with compromised systems in a structured, extensible, and scriptable way—supporting both Linux and Windows targets.

## Features

- **Session Management:**  
  Manage multiple TCP and UDP sessions with remote hosts, including interactive shells and backgrounding.
- **Modular Architecture:**  
  Easily extend functionality by adding new modules for Linux and Windows. Modules are hot-swappable and organized by platform and category.
- **Built-in Modules:**  
  - File transfer (download/upload) for Linux and Windows
  - System information (`whoami`, `uname`, `systeminfo`, `tasklist`, `netstat`)
  - Privilege escalation checks (LinPEAS, WinPEAS, sudo checks)
  - Interactive shell upgrades (PTY, stty, etc.)
  - Script runner for automating command sequences
- **Live Output Streaming:**  
  See command output as it arrives, even for long-running scripts.
- **Marker-based Output Handling:**  
  Reliably capture command output using end markers, even over unreliable or delayed connections.
- **Scriptable CLI:**  
  Interactive shell with command history, tab completion, and the ability to run scripts of commands on sessions for automation.
- **Cross-Platform:**  
  Works with both Linux and Windows targets, supporting common post-exploitation workflows.
- **Extensible:**  
  Add your own modules for enumeration, persistence, lateral movement, or custom tasks.
- **Robust UDP & TCP Support:**  
  Handles connectionless shells and tracks sessions by remote address to prevent duplication.
- **Operator Quality-of-Life:**  
  Colored logging, clear error messages, and modular codebase for easy contributions.

## Example Usage

```
run-module linux/download 1 /etc/passwd
run-module windows/systeminfo 2
run-module linux/linpeas 1
run-module windows/winpeas 2
interact 1
run-script 1 myscript.txt
```

## Project Structure

- `src/core/modules/` — All modules (Linux and Windows)
- `src/cli/` — Command-line interface and REPL logic
- `src/core/comms/` — Communication layer (TCP/UDP)
- `src/core/session.rs` — Session management

## Status

**This project is a work in progress.**  
Expect breaking changes, incomplete features, and evolving APIs.  
Contributions, feedback, and module ideas are welcome!

---

## Disclaimer

**Aegis is provided for educational and authorized security testing purposes only.**  
The author holds **no liability** for any misuse or damage caused by this tool.  
**You are solely responsible for ensuring you have proper authorization before using Aegis on any system.**

---

**Aegis** aims to be a modern, operator-friendly post-exploitation toolkit—fast, reliable, and easy to extend.