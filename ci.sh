#!/bin/bash

# URL to download the jimmer binary from the GitHub repository's 'bin' folder
JIMMER_URL="https://raw.githubusercontent.com/skubed0007/jimmer/master/bin/jimmer-linux-gnu-x86_64"

# Function to check if the download was successful
check_download_success() {
    if [ ! -f "$1" ]; then
        echo "Error: Failed to download the file."
        exit 1
    fi
}

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run as root or use sudo."
    exit 1
fi

# Create /usr/local/bin if it doesn't exist
if [ ! -d "/usr/local/bin" ]; then
    echo "Creating /usr/local/bin directory..."
    sudo mkdir -p /usr/local/bin
fi

# Remove existing jimmer binary if it already exists
if [ -f "/usr/local/bin/jimmer" ]; then
    echo "Removing existing jimmer binary at /usr/local/bin/jimmer..."
    sudo rm /usr/local/bin/jimmer
fi

# Download the jimmer binary
echo "Downloading jimmer from $JIMMER_URL..."
tmp_file=$(mktemp)
sudo curl -L -o "$tmp_file" "$JIMMER_URL"

# Check if the download was successful
check_download_success "$tmp_file"

# Move the downloaded binary to /usr/local/bin and make it executable
echo "Installing jimmer to /usr/local/bin..."
sudo mv "$tmp_file" /usr/local/bin/jimmer
sudo chmod +x /usr/local/bin/jimmer

echo "Installation completed. 'jimmer' has been installed to /usr/local/bin."
