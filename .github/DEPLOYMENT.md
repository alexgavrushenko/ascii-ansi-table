# GitHub Pages Deployment Guide

This repository includes an automated GitHub Actions workflow that builds the WASM package and deploys the test suite to GitHub Pages.

## Setup Instructions

### 1. Enable GitHub Pages

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Pages**
3. Under "Source", select **GitHub Actions**
4. The workflow will automatically deploy on pushes to `main` or `master` branch

### 2. Access Your Test Suite

After the first successful deployment, your test suite will be available at:
```
https://YOUR-USERNAME.github.io/YOUR-REPOSITORY-NAME/
```

Example: `https://yourusername.github.io/ascii-ansi-table-wasm/`

### 3. Update README

Don't forget to update the README.md link to point to your actual GitHub Pages URL:
```markdown
**🌐 [View Live Test Suite](https://yourusername.github.io/ascii-ansi-table-wasm/)**
```

## Workflow Features

The GitHub Actions workflow includes:

### Build Process
- ✅ Rust toolchain setup with WASM target
- ✅ wasm-pack installation and build
- ✅ Rust tests, clippy, and formatting checks
- ✅ WASM package generation for web target in `html/pkg`

### Deployment
- ✅ Automatic deployment to GitHub Pages on main/master branch pushes
- ✅ Interactive test suite with comprehensive coverage
- ✅ Beautiful landing page with feature overview
- ✅ ANSI to HTML conversion demos
- ✅ Performance benchmarking tools

### Test Coverage
- ✅ **Comprehensive Tests**: Multi-line cell rendering, text wrapping, height calculation
- ✅ **Performance Tests**: Large table generation (1000+ rows) with complex content
- ✅ **ANSI Support**: Beautiful color formatting converted to HTML
- ✅ **Streaming Demo**: Real-time table generation examples

## Manual Deployment

If you need to deploy manually:

```bash
# Build WASM package
wasm-pack build --target web --out-dir html/pkg --dev

# Create deployment directory
mkdir -p pages-deploy
cp -r html/ pages-deploy/

# Deploy to your hosting service
```

## Troubleshooting

### Workflow Fails
- Check that GitHub Actions are enabled in your repository settings
- Ensure the repository is public or you have GitHub Pro/Team for private repo Pages
- Verify the workflow has proper permissions (should be automatic)

### Tests Fail
- The workflow runs Rust tests before deployment
- Check the "Actions" tab for detailed error logs
- Ensure all tests pass locally with `cargo test`

### Pages Not Loading
- Check that GitHub Pages is configured to use "GitHub Actions" as source
- Wait a few minutes after the first deployment
- Verify the workflow completed successfully in the Actions tab

## Repository Structure

```
.github/
  workflows/
    build-and-deploy.yml    # Main CI/CD workflow
  DEPLOYMENT.md            # This guide
html/
  comprehensive-test.html  # Main test runner
  ansi-demo.html          # ANSI color demonstrations
  streaming-demo.html     # Streaming examples
  test-suite-performance.js # Performance benchmarks
  pkg/                    # WASM package for web
src/                      # Rust source code
```