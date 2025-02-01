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

# Function to download a file
download_file() {
    local url=$1
    local destination=$2
    echo "Downloading $url..."
    curl -L -o "$destination" "$url"
    check_download_success "$destination"
}

# Create ~/jimmer directory if it doesn't exist
USER_HOME=$(eval echo ~$SUDO_USER)
JIMMER_DIR="$USER_HOME/jimmer"
if [ ! -d "$JIMMER_DIR" ]; then
    echo "Creating $JIMMER_DIR directory..."
    mkdir -p "$JIMMER_DIR"
fi

# Download the audio file to the ~/jimmer directory
MP3_FILE="$JIMMER_DIR/audio.mp3"
if [ ! -f "$MP3_FILE" ]; then
    download_file "$MP3_URL" "$MP3_FILE"
fi

# Check if the jimmer binary exists in /usr/local/bin
if [ ! -f "/usr/local/bin/jimmer" ]; then
    # Prompt for root privileges to install the jimmer binary
    echo "The jimmer binary is not installed. Installing to /usr/local/bin..."
    sudo curl -L -o /usr/local/bin/jimmer "$JIMMER_URL"
    sudo chmod +x /usr/local/bin/jimmer
    echo "jimmer has been installed to /usr/local/bin."
else
    echo "jimmer is already installed at /usr/local/bin."
fi

echo "Installation completed. 'jimmer' has been installed to /usr/local/bin and 'audio.mp3' to $USER_HOME/jimmer."
