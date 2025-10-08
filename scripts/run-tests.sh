#!/bin/bash

set -e

echo "Starting Selenium Chrome container..."
docker compose -f docker-compose.test.yml up -d

echo "Waiting for Selenium to be ready..."
timeout=30
elapsed=0
while ! curl -sf http://localhost:4444/wd/hub/status > /dev/null; do
    if [ $elapsed -ge $timeout ]; then
        echo "Timeout waiting for Selenium"
        docker compose -f docker-compose.test.yml logs
        docker compose -f docker-compose.test.yml down
        exit 1
    fi
    sleep 1
    elapsed=$((elapsed + 1))
done

echo "Selenium is ready!"

# Run the tests
echo "Running tests..."
SELENIUM_URL=http://localhost:4444 cargo test --test e2e -- --nocapture

# Capture exit code
TEST_EXIT_CODE=$?

# Cleanup
echo "Cleaning up..."
docker compose -f docker-compose.test.yml down

exit $TEST_EXIT_CODE
