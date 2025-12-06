# Zenith Test Suite

Comprehensive testing infrastructure for Zenith Data Plane.

## Test Structure

```
tests/
├── test_integration.py   # Integration tests (Python SDK)
├── run_e2e.sh           # End-to-end test runner
└── README.md            # This file
```

## Running Tests

### Quick Test (All Components)
```bash
./tests/run_e2e.sh
```

This runs:
1. ✅ Core library build
2. ✅ WASM plugin build
3. ✅ Python SDK integration tests
4. ✅ Storage layer tests
5. ✅ Runtime tests
6. ✅ Host API tests
7. ✅ Demo application

### Individual Test Suites

**Python Integration Tests:**
```bash
cd tests
python3 test_integration.py
```

**Storage Tests:**
```bash
cd storage
cargo test
```

**Runtime Tests:**
```bash
cd runtime
cargo test --lib
```

**Host API Tests:**
```bash
cd host-api
cargo test -- --test-threads=1
```

## Test Coverage

| Component | Test Type | Coverage |
|-----------|-----------|----------|
| Core Engine | Unit | Basic |
| Python SDK | Integration | Full |
| Storage | Unit | Full |
| Runtime | Unit | Full |
| Host API | Unit | Full |
| WASM Plugins | Build | Full |

## CI/CD Integration

Tests are automatically run on GitHub Actions:

```yaml
- name: Run E2E Tests
  run: ./tests/run_e2e.sh
```

## Writing New Tests

### Python Integration Test
```python
class TestMyFeature(unittest.TestCase):
    def setUp(self):
        self.client = ZenithClient()
    
    def test_feature(self):
        # Your test here
        pass
```

### Rust Unit Test
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // Your test here
    }
}
```

## Performance Testing

See `tools/benchmark.py` for performance benchmarks.

## Test Results

Expected output from `run_e2e.sh`:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Test Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Passed: 7
Failed: 0

🎉 All tests passed!
```

## Troubleshooting

**Build Failures:**
- Ensure Rust toolchain is installed
- Run `rustup target add wasm32-wasip1`

**Python Test Failures:**
- Install dependencies: `pip install pyarrow`
- Ensure core library is built: `cd core && cargo build --release`

**Permission Errors:**
- Make scripts executable: `chmod +x tests/*.sh`
