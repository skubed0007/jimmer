#!/bin/bash

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run as root or use sudo."
    exit 1
fi

# Create /usr/local/bin if it doesn't exist
if [ ! -d "/usr/local/bin" ]; then
    sudo mkdir -p /usr/local/bin
fi
if [ -f "/usr/local/bin/jimmer" ]; then
    echo "Removing existing jimmer binary at /usr/local/bin/jimmer..."
    sudo rm /usr/local/bin/jimmer
fi

# Copy the jimmer-linux-gnu-x86_64 file to /usr/local/bin
sudo cp jimmer-linux-gnu-x86_64 /usr/local/bin/jimmer

# Make the file executable
sudo chmod +x /usr/local/bin/jimmer

echo "Installation completed. 'jimmer' has been copied to /usr/local/bin."