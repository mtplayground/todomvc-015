#!/bin/sh
set -e

# Ensure data directory exists
mkdir -p /app/data

# Start the backend server
exec /app/backend
