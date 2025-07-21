# Coding Conventions

## General Principles

1. **Fix bugs at the source**: If a bug is found in Rust code, fix it in Rust and add comprehensive tests in Rust. Do not work around issues in other layers.

2. **Follow existing patterns**: Always examine the codebase first to understand existing conventions, libraries, and architectural patterns before making changes.

3. **Security first**: Never introduce code that exposes or logs secrets and keys. Never commit secrets or keys to the repository.

## Code Style

### Comments

- **NO inline comments** inside functions or struct fields
- All comments must be placed **before** functions/structs according to Rust documentation standards
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Use `//` for implementation notes before functions/structs

```rust
// WRONG - inline comments banned
fn calculate_width() -> usize {
    let width = 10; // This is the default width
    width + 2 // Add padding
}

// CORRECT - comments before function
/// Calculates the output width including padding
fn calculate_width() -> usize {
    let width = 10;
    width + 2
}
```

### Conditional Compilation

- Use file-level `#![cfg(feature = "...")]` instead of individual item-level attributes when the entire file is feature-specific
- Place feature gates at the module level in `mod.rs` files

```rust
// WRONG - many individual attributes
#[cfg(feature = "wasm")]
pub struct WasmTable {}

#[cfg(feature = "wasm")]
impl WasmTable {}

// CORRECT - file-level attribute
#![cfg(feature = "wasm")]

pub struct WasmTable {}
impl WasmTable {}
```

### Testing

- Every bug fix must include a test that reproduces the issue
- Test edge cases, especially for text processing (empty strings, newlines, Unicode)
- Use descriptive test names that explain the scenario being tested
- Include `println!` debugging in tests when helpful for understanding behavior

### Build System

- Use consistent output directories (`html/pkg` for WASM builds)
- Verify both regular builds (`cargo build`) and feature builds (`wasm-pack build`) work
- Update import paths consistently across all HTML files when build structure changes

### Code Organization

- Keep related functionality grouped in appropriate modules
- Use clear, descriptive function and variable names
- Prefer editing existing files over creating new ones
- Remove dead code and unused dependencies

### Error Handling

- Use proper Rust error handling patterns (`Result`, `?` operator)
- Provide meaningful error messages
- Convert errors appropriately at layer boundaries (e.g., Rust errors to `JsValue` in WASM bindings)

### Performance

- Test with realistic data sizes (e.g., 1000 rows with complex content)
- Profile and measure performance changes
- Use appropriate data structures for the use case

## File Structure

- Source code in `src/`
- Tests in `#[cfg(test)]` modules within source files
- WASM build output in `html/pkg/`
- HTML demos and tests in `html/`

## Commit Standards

- Fix the root cause, not symptoms
- Include tests with bug fixes
- Run both `cargo build` and `wasm-pack build` before committing
- Keep commits focused and atomic