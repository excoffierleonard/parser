#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="parser.excoffierleonard.com"

# Function to print test results
print_result() {
    local test_name=$1
    local result=$2
    if [ $result -eq 0 ]; then
        echo -e "${GREEN}✓ $test_name passed${NC}"
    else
        echo -e "${RED}✗ $test_name failed${NC}"
        exit 1
    fi
}

echo "Starting deployment tests..."

# Test 1: Hello endpoint
echo -e "\nTesting /hello/test_name endpoint..."
response=$(curl -s "$BASE_URL/hello/test_name")
expected='{"message":"Hello test_name!"}'

if [ "$response" == "$expected" ]; then
    print_result "Hello endpoint test" 0
else
    echo -e "${RED}Expected: $expected${NC}"
    echo -e "${RED}Got: $response${NC}"
    print_result "Hello endpoint test" 1
fi

# Test 2: Frontend serving
echo -e "\nTesting frontend serving..."
# Get the actual content instead of just headers
response=$(curl -s "$BASE_URL")

# Check if the response contains typical HTML tags
if echo "$response" | grep -q "<html" && echo "$response" | grep -q "<body"; then
    print_result "Frontend serving test" 0
else
    echo -e "${RED}Expected HTML content in response${NC}"
    echo -e "${RED}First 100 characters of response: ${response:0:100}${NC}"
    print_result "Frontend serving test" 1
fi

echo -e "\nAll tests completed successfully!"