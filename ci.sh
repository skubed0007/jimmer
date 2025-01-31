#!/bin/bash

# URLs to download the jimmer binary and audio file from the GitHub repository's 'bin' folder
JIMMER_URL="https://raw.githubusercontent.com/skubed0007/jimmer/master/bin/jimmer-linux-gnu-x86_64"
MP3_URL="https://raw.githubusercontent.com/skubed0007/jimmer/master/audio.mp3"

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

# Remove existing jimmer binary and audio file if they already exist
if [ -f "/usr/local/bin/jimmer" ]; then
    echo "Removing existing jimmer binary at /usr/local/bin/jimmer..."
    sudo rm /usr/local/bin/jimmer
fi

if [ -f "/usr/local/bin/audio.mp3" ]; then
    echo "Removing existing audio file at /usr/local/bin/audio.mp3..."
    sudo rm /usr/local/bin/audio.mp3
fi

# Download the jimmer binary
echo "Downloading jimmer from $JIMMER_URL..."
tmp_file=$(mktemp)
sudo curl -L -o "$tmp_file" "$JIMMER_URL"

# Check if the download was successful
check_download_success "$tmp_file"

# Move the downloaded jimmer binary to /usr/local/bin and make it executable
echo "Installing jimmer to /usr/local/bin..."
sudo mv "$tmp_file" /usr/local/bin/jimmer
sudo chmod +x /usr/local/bin/jimmer

# Download the audio file
echo "Downloading audio file from $MP3_URL..."
mp3_tmp_file=$(mktemp)
sudo curl -L -o "$mp3_tmp_file" "$MP3_URL"

# Check if the download was successful
check_download_success "$mp3_tmp_file"

# Move the downloaded audio file to /usr/local/bin
echo "Installing audio file to /usr/local/bin..."
sudo mv "$mp3_tmp_file" /usr/local/bin/audio.mp3

echo "Installation completed. 'jimmer' and 'audio.mp3' have been installed to /usr/local/bin."
