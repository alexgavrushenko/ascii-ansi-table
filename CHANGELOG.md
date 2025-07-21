# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.1] - 2024-07-18

### Added
- Initial alpha release
- Core table rendering functionality
- ANSI color support with proper wrapping
- Unicode and ASCII border styles (honeywell, norc, ramac, void)
- Text wrapping with word-based and character-based options
- Column configuration (width, alignment, padding, truncation)
- Spanning cells support
- Streaming API for large datasets
- CLI interface with interactive streaming demo
- WASM bindings for browser/Node.js usage
- Auto-calculated column widths
- Multiple alignment options (horizontal and vertical)
- Comprehensive test suite
- Performance benchmarks

### Features
- **Core**: Basic table rendering with configurable borders
- **ANSI Support**: Proper handling of ANSI escape sequences in cell content
- **Text Processing**: Intelligent wrapping, alignment, and truncation
- **Streaming**: Memory-efficient streaming for large tables
- **CLI**: Command-line tool with streaming demo
- **WASM**: WebAssembly bindings for web usage
- **Configuration**: Flexible column and table configuration options

### Known Limitations
- API may change in future versions
- Limited performance optimization
- Documentation needs expansion
- Some edge cases in ANSI handling may exist

### Notes
- This is an early alpha release
- Inspired by the JavaScript library [gajus/table](https://github.com/gajus/table)
- Complete rewrite in Rust with substantial new features
- Not recommended for production use yet

[Unreleased]: https://github.com/your-username/ascii-ansi-table/compare/v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/your-username/ascii-ansi-table/releases/tag/v0.1.0-alpha.1