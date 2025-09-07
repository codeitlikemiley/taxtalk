#!/bin/bash

# Web development server script with configurable ports
# Usage: ./serve.sh
# Environment variables:
#   SERVER_PORT - Backend API server port (default: 3000)
#   CLIENT_PORT - Frontend dev server port (default: 8080)

# Set default values
CLIENT_PORT=${CLIENT_PORT:-8080}
SERVER_PORT=${SERVER_PORT:-3000}

echo "Starting TaxTalk web dev server..."
echo "  Client port: $CLIENT_PORT"
echo "  API server port: $SERVER_PORT"

# Export SERVER_PORT for the build to use
export SERVER_PORT

# Run trunk serve with the specified port
trunk serve --port $CLIENT_PORT