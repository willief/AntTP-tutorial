# Contributing to AntTP Tutorial

Thank you for your interest in contributing! This guide will help you get started.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Areas for Contribution](#areas-for-contribution)

## Code of Conduct

Please be respectful and constructive in all interactions. We're building an educational tool to help developers learn - let's keep it welcoming and inclusive.

## Getting Started

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/anttp-tutorial.git
   cd anttp-tutorial
   ```
3. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Docker & Docker Compose
- Rust 1.93+ (for backend development)
- Node.js 20+ (for frontend development)
- Git

### Local Development (Without Docker)

**Backend:**
```bash
cd backend
cargo run
# Runs on http://localhost:8080
```

**Frontend:**
```bash
cd frontend
npm install
npm run dev
# Runs on http://localhost:5173
```

### Docker Development

```bash
# Full rebuild
./rebuild.sh

# Or individual services
docker compose build backend
docker compose up -d backend
```

## Making Changes

### Project Structure

```
anttp-tutorial/
├── backend/
│   ├── src/main.rs          # API implementation
│   └── Cargo.toml           # Dependencies
├── frontend/
│   ├── src/
│   │   ├── routes/          # Pages
│   │   ├── app.css          # Styles
│   │   └── +layout.svelte   # Layout
│   └── package.json
└── docs/                    # Documentation
```

### Coding Standards

**Rust (Backend):**
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Add comments for complex logic
- Keep functions under 50 lines where possible

```bash
cargo fmt
cargo clippy
```

**Svelte (Frontend):**
- Use consistent indentation (2 spaces)
- Keep components under 300 lines
- Extract reusable components
- Use meaningful variable names

**General:**
- Write descriptive commit messages
- Add tests for new features
- Update documentation
- Keep PRs focused and small

### Commit Messages

Follow conventional commits:

```
feat: Add PNR type selector dropdown
fix: Resolve backend crash on empty chunk
docs: Update API reference for registers
refactor: Extract sidebar component
test: Add integration tests for archives
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Testing

### Backend Tests

```bash
cd backend
cargo test
```

### API Tests

```bash
# Full test suite
./test-all-api.sh

# Individual endpoint
curl -X POST http://localhost:8080/api/chunks \
  -H "Content-Type: application/json" \
  -d '{"content":"test"}'
```

### Frontend Testing

```bash
cd frontend
npm test
```

### Manual Testing Checklist

Before submitting:
- [ ] All 6 primitive pages load
- [ ] Forms submit successfully
- [ ] Data displays correctly
- [ ] Sidebar navigation works
- [ ] Mobile responsive (test at 375px, 768px, 1024px)
- [ ] No console errors
- [ ] API endpoints return correct data

## Submitting Changes

### Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create Pull Request**:
   - Clear title describing the change
   - Reference any related issues
   - Include screenshots for UI changes
   - List testing performed

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature  
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Backend tests pass
- [ ] API tests pass
- [ ] Manual testing completed
- [ ] Tested on mobile

## Screenshots
(if applicable)

## Related Issues
Closes #123
```

### Review Process

- Maintainers will review within 1-2 weeks
- Address feedback in new commits
- Once approved, changes will be merged
- Your contribution will be credited

## Areas for Contribution

### High Priority

**Phase 1 Improvements:**
- [ ] Additional frontend examples
- [ ] Better error messages
- [ ] Loading states and animations
- [ ] Input validation improvements
- [ ] Accessibility enhancements

**Documentation:**
- [ ] Video tutorials
- [ ] More code examples
- [ ] Troubleshooting guide expansion
- [ ] Translation to other languages

**Testing:**
- [ ] Increase test coverage
- [ ] E2E tests with Playwright
- [ ] Performance benchmarks
- [ ] Load testing

### Phase 2 (Network Integration)

See [ROADMAP.md](ROADMAP.md) for detailed plans:
- [ ] Autonomi SDK integration
- [ ] Real data storage
- [ ] EVM payment handling
- [ ] CLI tool compatibility

**Good First Issues:**
- Look for issues labeled `good-first-issue`
- Usually documentation or small feature additions
- Great for getting familiar with the codebase

### UI/UX Improvements

- Better form validation messages
- Dark mode support
- Data visualization for registers
- File upload progress bars
- Export/import functionality

### Performance

- Frontend bundle optimization
- API response caching
- Connection pooling
- Request batching

## Development Tips

### Backend Hot Reload

```bash
cargo install cargo-watch
cargo watch -x run
```

### Frontend Hot Reload

Already enabled with `npm run dev`

### Docker Logs

```bash
# All logs
docker compose logs -f

# Specific service
docker compose logs -f backend
docker compose logs -f frontend
```

### Debugging

**Backend:**
```rust
// Add debug prints
println!("Debug: {:?}", variable);
eprintln!("Error: {:?}", error);

// Use debugger
rust-lldb target/debug/anttp-tutorial-backend
```

**Frontend:**
```javascript
// Browser console
console.log('Debug:', variable);
console.error('Error:', error);

// Svelte devtools (browser extension)
```

## Documentation

When adding features, please update:

- [ ] README.md (if user-facing)
- [ ] API.md (if API changes)
- [ ] Code comments
- [ ] Examples
- [ ] CHANGELOG.md

## Questions?

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Discord/Community**: Link if available

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- README.md contributors section
- CHANGELOG.md for each release
- GitHub contributors page

Thank you for contributing to AntTP Tutorial! 🎉
