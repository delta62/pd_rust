#!/usr/bin/env bash
#
# Build a Linux library into a Playdate executable

set -e

if [ $# -lt 1 ]; then
    echo >&2 "Usage: $0 <example> [--run]"
    exit 1
fi

if [ ! -d "examples/${1}" ]; then
    echo >&2 "Example '${1}' not found. Pick a name from the examples/ directory."
    exit 1
fi

APP_NAME="$1"
BUILD_DIR="./examples/${APP_NAME}/target/debug"
BUILT_LIB="${BUILD_DIR}/lib${APP_NAME}.so"
SOURCE_DIR="Source"
PLAYDATE_SDK_PATH="${HOME}/.local/share/playdate-sdk"
CLEAN_FILES=("${SOURCE_DIR}/pdex.so")

function pre_build() {
    if [ -d "${SOURCE_DIR}" ]; then
        echo "Cleaning build artifacts"

        for CLEAN_FILE in "${CLEAN_FILES[@]}"; do
            if [ -f "${CLEAN_FILE}" ]; then
                echo "Removing ${CLEAN_FILE}"
                rm "${CLEAN_FILE}"
            fi
        done
    else
        echo "Creating ${SOURCE_DIR}"
        mkdir "${SOURCE_DIR}"
    fi
}

function build() {
    # build runtime
    cargo build

    # build app
    pushd "examples/${APP_NAME}" > /dev/null
    cargo build
    popd

    echo "Copying .so -> pdex.so"
    cp "${BUILT_LIB}" "${SOURCE_DIR}/pdex.so"

    echo "Compiling pdx with pdc"
    "${PLAYDATE_SDK_PATH}/bin/pdc" \
        -sdkpath "${PLAYDATE_SDK_PATH}" \
        "${SOURCE_DIR}" \
        "${APP_NAME}.pdx"

    echo "Compiled ${APP_NAME}.pdx"
}

pre_build
build

if [ "$1" = "--run" ] || [ "$2" = "--run" ]; then
    "${PLAYDATE_SDK_PATH}/bin/PlaydateSimulator" "${APP_NAME}.pdx"
fi
