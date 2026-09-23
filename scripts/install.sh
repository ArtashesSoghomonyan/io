#!/bin/sh

# tells a shell script to exit immediately if any command fails or if it tries to use an unset variable
set -eu

REPO="ArtashesSoghomonyan/io"
INSTALL_DIR="/usr/local/bin"

OS="$(uname -s)"
ARCH="$(uname -m)"

# Step1: Check the os and architecture
if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "aarch64" ]; then
        RELEASE="io-aarch64-unknown-linux-gnu.tar.gz"
    elif [ "$ARCH" = "x86_64" ]; then
        RELEASE="io-x86_64-unknown-linux-gnu.tar.gz"
    else
        echo "Sorry your system's architecture is not supported yet."
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        RELEASE="io-aarch64-apple-darwin.tar.gz"
    elif [ "$ARCH" = "x86_64" ]; then
        RELEASE="io-x86_64-apple-darwin.tar.gz"
    else
        echo "Sorry your system's architecture is not supported yet."
        exit 1
    fi
else
    echo "Sorry your os is not supported yet."
    echo "However if you are using x86_64 windows you can still install it manually"
    echo "  with .exe file on https://github.com/$REPO/releases/latest"
    exit 1
fi

# Step2: Install binary release
latest_tag=$(curl -s https://api.github.com/repos/$REPO/releases/latest \
  | grep '"tag_name"' \
  | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')

echo $latest_tag

# tar -xvf "" -C /path/to/destination
curl -LJO "https://github.com/$REPO/releases/download/$latest_tag/$RELEASE"

tar -xvf "$RELEASE" -C "$INSTALL_DIR"

rm "$RELEASE"

chmod 777 "$INSTALL_DIR/io"

# Step3: Install default io config

curl -LJO "https://raw.githubusercontent.com/$REPO/refs/heads/main/io.config.toml"

mv io.config.toml ~

echo "Install completed!"
