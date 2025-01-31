#!/bin/bash

# URL to download the jimmer binary
JIMMER_URL="https://raw.githubusercontent.com/skubed0007/jimmer/master/bin/jimmer-linux-gnu-x86_64"

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run as root or use sudo."
    exit 1
fi

# Create /usr/local/bin if it doesn't exist
if [ ! -d "/usr/local/bin" ]; then
    sudo mkdir -p /usr/local/bin
fi
# Remove existing jimmer binary if it already exists
if [ -f "/usr/local/bin/jimmer" ]; then
    echo "Removing existing jimmer binary at /usr/local/bin/jimmer..."
    sudo rm /usr/local/bin/jimmer
fi
# Download the jimmer binary
echo "Downloading jimmer from $JIMMER_URL..."
sudo curl -L -o /usr/local/bin/jimmer $JIMMER_URL

# Make the binary executable
sudo chmod +x /usr/local/bin/jimmer

echo "Installation completed. 'jimmer' has been installed to /usr/local/bin."