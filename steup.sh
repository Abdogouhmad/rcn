#!/usr/bin/env bash

# Color constants
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Global constants
REPO_URL="https://github.com/Abdogouhmad/rcn"
BIN_PATH="/usr/bin/rcn"

# Print messages with color
printcl() {
    local color=$1
    local message=$2
    printf "%b%s%b\n" "${color}" "${message}" "${NC}"
}

# Install the rcn CLI
install_git_conform() {
    printcl "${YELLOW}" "Installing The CLI into ${BIN_PATH}..."

    # Download the binary
    if ! sudo wget -q --show-progress -O "${BIN_PATH}" "${REPO_URL}/releases/latest/download/rcn"; then
        printcl "${YELLOW}" "Error: Failed to download the binary."
        return 1
    fi

    # Make the binary executable
    if ! sudo chmod +x "${BIN_PATH}"; then
        printcl "${YELLOW}" "Error: Failed to make rcn executable."
        return 1
    fi

    printcl "${GREEN}" "Installed Successfully! Run the CLI with the command: rcn help"
}

# Uninstall the rcn CLI
uninstall_git_conform() {
    printcl "${YELLOW}" "Uninstalling The CLI from ${BIN_PATH} and ${LOCAL_SHARE_PATH}..."

    # Remove the binary
    if ! sudo rm -i "${BIN_PATH}"; then
        printcl "${YELLOW}" "Error: Failed to remove the binary at ${BIN_PATH}."
        return 1
    fi


    printcl "${GREEN}" "Uninstalled Successfully!"
}

# Main function to choose an action
main() {
    printcl "${YELLOW}" "Choose an action:"
    printcl "${YELLOW}" "1: Install"
    printcl "${YELLOW}" "2: Uninstall"
    read -r -p "Enter your choice: " choice

    case "$choice" in
        1)
            install_git_conform
            ;;
        2)
            uninstall_git_conform
            ;;
        *)
            printcl "${YELLOW}" "Invalid choice. Please choose 1 to install or 2 to uninstall."
            exit 1
            ;;
    esac
}

# Run the main function
main

exit 0
