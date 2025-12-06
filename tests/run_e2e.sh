#!/bin/bash
# End-to-end test script
# Tests complete Zenith workflow

set -e

echo "🧪 Zenith End-to-End Test Suite"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass_count=0
fail_count=0

run_test() {
    local test_name=$1
    local test_cmd=$2
    
    echo -n "Testing: $test_name... "
    
    if eval "$test_cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((pass_count++))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((fail_count++))
        return 1
    fi
}

echo ""
echo "Step 1: Build Core"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Core library build" "cd core && cargo build --release 2>&1"

echo ""
echo "Step 2: Build Plugin"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "WASM plugin build" "cd plugins/simple_filter && cargo build --target wasm32-wasip1 --release 2>&1"

echo ""
echo "Step 3: Python SDK Tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Python integration tests" "cd tests && python3 test_integration.py 2>&1"

echo ""
echo "Step 4: Storage Tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Storage layer tests" "cd storage && cargo test 2>&1"

echo ""
echo "Step 5: Runtime Tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Runtime tests" "cd runtime && cargo test --lib 2>&1"

echo ""
echo "Step 6: Host API Tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Host API tests" "cd host-api && cargo test -- --test-threads=1 2>&1"

echo ""
echo "Step 7: Demo Application"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
run_test "Demo app execution" "python3 demo_app.py 2>&1"

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Test Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}Passed: $pass_count${NC}"
echo -e "${RED}Failed: $fail_count${NC}"

total=$((pass_count + fail_count))
if [ $fail_count -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed${NC}"
    exit 1
fi
