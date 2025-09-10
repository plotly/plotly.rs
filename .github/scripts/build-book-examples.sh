#!/bin/bash

# Script to build examples needed for book artifacts
# This script is used by both the CI build_book job and the book.yml workflow

# Note: static_export example requires Chrome setup for WebDriver functionality
# The CI jobs set up Chrome, while when running locally, you may need to setup 
# your browser and webdriver manually to ensure this example works properly.

set -e  # Exit on any error

# Function to display usage
usage() {
    echo "Usage: $0 <examples_directory>"
    echo "  examples_directory: Path to the examples directory (e.g., examples or \$GITHUB_WORKSPACE/examples)"
    echo ""
    echo "This script builds all examples needed for the book documentation."
    exit 1
}

# Check if examples directory is provided
if [ $# -ne 1 ]; then
    usage
fi

EXAMPLES_DIR="$1"

# Validate that the examples directory exists
if [ ! -d "$EXAMPLES_DIR" ]; then
    echo "Error: Examples directory '$EXAMPLES_DIR' does not exist"
    exit 1
fi

echo "Building examples for book artifacts from: $EXAMPLES_DIR"

# List of examples needed for the book, sorted alphabetically
# These examples generate HTML files that are included in the book documentation
BOOK_EXAMPLES=(
    "3d_charts"
    "basic_charts"
    "custom_controls"
    "financial_charts"
    "scientific_charts"
    "shapes"
    "static_export"
    "statistical_charts"
    "subplots"
    "themes"
    "downsampling"
)

# Build each example
for example in "${BOOK_EXAMPLES[@]}"; do
    example_path="$EXAMPLES_DIR/$example"
    
    if [ ! -d "$example_path" ]; then
        echo "Warning: Example directory '$example_path' does not exist, skipping..."
        continue
    fi
    
    echo "Building $example example..."
    cd "$example_path"
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        echo "Warning: No Cargo.toml found in $example_path, skipping..."
        cd - > /dev/null
        continue
    fi
    
    # Run the example
    if cargo run; then
        echo "✓ Successfully built $example"
    else
        echo "✗ Failed to build $example"
        exit 1
    fi
    
    # Return to the original directory
    cd - > /dev/null
done

echo "All examples built successfully!"
