#!/bin/bash

# Web build script with configurable server port
# Usage: ./build.sh
# Environment variables:
#   SERVER_PORT - Backend API server port (default: 3000)

# Set default values
SERVER_PORT=${SERVER_PORT:-3000}

echo "Building TaxTalk web app..."
echo "  API server port: $SERVER_PORT"

# Export SERVER_PORT for the build to use
export SERVER_PORT

# Build with trunk
trunk build --release