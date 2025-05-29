# Contributing to Aegis

Thank you for your interest in contributing to **Aegis**!  
Your help is welcome—whether it's code, documentation, bug reports, or feature ideas.

---

## Ways to Contribute

- **Report Bugs:**  
  Open a [GitHub Issue](../../issues) with steps to reproduce, expected/actual behavior, and environment details.
- **Suggest Features:**  
  Start a [Discussion](../../discussions) or open an issue to propose new modules or improvements.
- **Submit Pull Requests:**  
  Add new modules, fix bugs, improve documentation, or refactor code.
- **Improve Documentation:**  
  Help make Aegis easier to use and understand.

---

## Development Guidelines

### 1. Code Style

- Follow Rust best practices and idioms.
- Use clear, descriptive names for modules, functions, and variables.
- Keep code modular—prefer small, focused modules.
- Add comments for complex logic.

### 2. Adding a New Module

1. **Create a new file** in `src/core/modules/linux/` or `src/core/modules/windows/`.
2. **Implement the `Module` trait** (see existing modules for examples).
3. **Register your module** in `src/core/modules/registry.rs` by adding it to the list in `get_modules()`.
4. **Test your module** with both TCP and UDP sessions if possible.

### 3. Submitting a Pull Request

- Fork the repository and create a new branch for your changes.
- Write clear commit messages.
- Test your changes before submitting.
- Describe what your PR does and reference any related issues.

---

## Code of Conduct

- Be respectful and constructive.
- No offensive, illegal, or unethical content.
- Only contribute code and modules that you have the right to share.

---

## Legal Notice

**Aegis is for educational and authorized security testing only.**  
By contributing, you agree not to use this project for unauthorized or malicious purposes.

---

Thank you for helping make Aegis better!