#!/bin/bash
# Browser testing script

echo "Starting Minimalist Browser tests..."

# Test URLs for various features
TEST_URLS=(
    "minimalist://newtab"
    "minimalist://settings"
    "minimalist://memory"
    "minimalist://flash"
    "https://www.google.com"
    "https://html5test.com"
    "localhost:8080"
    "127.0.0.1:3000"
    "file:///tmp/test.html"
)

# Create test HTML file
cat > /tmp/test.html << 'EOF'
<!DOCTYPE html>
<html>
<head><title>Local File Test</title></head>
<body>
    <h1>Local File Test</h1>
    <p>This tests file:// protocol support.</p>
</body>
</html>
EOF

# Memory test function
test_memory() {
    echo "Testing memory usage..."
    ./target/release/minimalist-browser &
    BROWSER_PID=$!
    sleep 5
    
    for i in {1..10}; do
        MEM=$(ps -o rss= -p $BROWSER_PID 2>/dev/null | awk '{print $1/1024 " MB"}')
        echo "Memory after ${i}0 seconds: $MEM"
        sleep 10
    done
    
    kill $BROWSER_PID
}

# Run tests
echo "Testing URL handling..."
for url in "${TEST_URLS[@]}"; do
    echo "Testing: $url"
    timeout 5 ./target/release/minimalist-browser "$url" 2>/dev/null || true
done

# Memory test
test_memory

echo "Tests complete!"