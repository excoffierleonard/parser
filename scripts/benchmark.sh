#!/bin/bash

# Exit on any error
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}==== Parser Benchmark Script ====${NC}"

# Constants
TEST_FILES_DIR="./crates/core/tests/inputs"
WEB_API_URL="http://localhost:8080/parse"
ITERATIONS=5
WEB_SERVER_PID=""

# Function to check if command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Check if required commands are installed
if ! command_exists curl; then
  echo "Error: curl is not installed. Please install curl to run this benchmark."
  exit 1
fi

if ! command_exists bc; then
  echo "Error: bc is not installed. Please install bc to run this benchmark."
  exit 1
fi

# Build the release versions
echo -e "${BLUE}Building release versions...${NC}"
cargo build --release --workspace

# Prepare list of test files
TEST_FILES=("$TEST_FILES_DIR"/*.*) 
NUM_FILES=${#TEST_FILES[@]}
echo -e "${BLUE}Found $NUM_FILES test files for benchmarking${NC}"

# Start the web server
echo -e "${BLUE}Starting web API server...${NC}"
cargo run --release -p parser-web &
WEB_SERVER_PID=$!

# Wait for server to start
echo -e "${YELLOW}Waiting for web server to start...${NC}"
sleep 3

# Ensure server is shut down on exit
trap 'echo "Shutting down web server..."; kill $WEB_SERVER_PID 2>/dev/null' EXIT

# Run CLI benchmarks
echo -e "\n${GREEN}=== CLI Benchmark ===${NC}"
CLI_TIMES=()

for i in $(seq 1 $ITERATIONS); do
  echo -e "${YELLOW}CLI Iteration $i/$ITERATIONS${NC}"
  
  # Use the time command to measure execution time
  { time -p ./target/release/parser-cli "${TEST_FILES[@]}" > /dev/null; } 2> temp_time.txt
  
  # Extract real time from the output
  REAL_TIME=$(grep "real" temp_time.txt | awk '{print $2}')
  CLI_TIMES+=($REAL_TIME)
  
  echo "  Time: ${REAL_TIME}s"
done

# Run Web API benchmarks 
echo -e "\n${GREEN}=== Web API Benchmark ===${NC}"
WEB_TIMES=()

for i in $(seq 1 $ITERATIONS); do
  echo -e "${YELLOW}Web API Iteration $i/$ITERATIONS${NC}"
  
  # Create form data with all test files
  FORM_ARGS=()
  for file in "${TEST_FILES[@]}"; do
    FORM_ARGS+=(-F "file=@$file")
  done
  
  # Use the time command to measure execution time
  { time -p curl -s "${FORM_ARGS[@]}" $WEB_API_URL > /dev/null; } 2> temp_time.txt
  
  # Extract real time from the output
  REAL_TIME=$(grep "real" temp_time.txt | awk '{print $2}')
  WEB_TIMES+=($REAL_TIME)
  
  echo "  Time: ${REAL_TIME}s"
done

# Clean up temp file
rm -f temp_time.txt

# Calculate statistics for CLI
echo -e "\n${GREEN}=== Results ===${NC}"
echo -e "${BLUE}CLI Performance (seconds):${NC}"
echo "  Times: ${CLI_TIMES[*]}"
CLI_TOTAL=0
CLI_MIN=${CLI_TIMES[0]}
CLI_MAX=${CLI_TIMES[0]}

for t in "${CLI_TIMES[@]}"; do
  CLI_TOTAL=$(echo "$CLI_TOTAL + $t" | bc -l)
  
  # Check for min
  if (( $(echo "$t < $CLI_MIN" | bc -l) )); then
    CLI_MIN=$t
  fi
  
  # Check for max
  if (( $(echo "$t > $CLI_MAX" | bc -l) )); then
    CLI_MAX=$t
  fi
done

CLI_AVG=$(echo "scale=3; $CLI_TOTAL / $ITERATIONS" | bc -l)
echo "  Min: ${CLI_MIN}s"
echo "  Max: ${CLI_MAX}s" 
echo "  Avg: ${CLI_AVG}s"

# Calculate statistics for Web
echo -e "\n${BLUE}Web API Performance (seconds):${NC}"
echo "  Times: ${WEB_TIMES[*]}"
WEB_TOTAL=0
WEB_MIN=${WEB_TIMES[0]}
WEB_MAX=${WEB_TIMES[0]}

for t in "${WEB_TIMES[@]}"; do
  WEB_TOTAL=$(echo "$WEB_TOTAL + $t" | bc -l)
  
  # Check for min
  if (( $(echo "$t < $WEB_MIN" | bc -l) )); then
    WEB_MIN=$t
  fi
  
  # Check for max
  if (( $(echo "$t > $WEB_MAX" | bc -l) )); then
    WEB_MAX=$t
  fi
done

WEB_AVG=$(echo "scale=3; $WEB_TOTAL / $ITERATIONS" | bc -l)
echo "  Min: ${WEB_MIN}s"
echo "  Max: ${WEB_MAX}s"
echo "  Avg: ${WEB_AVG}s"

# Compare the two approaches
echo -e "\n${GREEN}=== Comparison ===${NC}"
if (( $(echo "$CLI_AVG > $WEB_AVG" | bc -l) )); then
  RATIO=$(echo "scale=2; $CLI_AVG / $WEB_AVG" | bc -l)
  echo -e "Web API is ${RATIO}x faster than CLI"
else
  RATIO=$(echo "scale=2; $WEB_AVG / $CLI_AVG" | bc -l)
  echo -e "CLI is ${RATIO}x faster than Web API"
fi

echo -e "\n${GREEN}Benchmark complete!${NC}"