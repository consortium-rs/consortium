#!/bin/sh
#
# reload_cm33.sh — stop the Cortex-M coprocessor, drop the old firmware,
# wait for a new one to be scp'd in from elsewhere, then restart it.
#
# Usage: ./reload_cm33.sh
#

set -e

RPROC_BASE="/sys/class/remoteproc"
FW_DIR="/lib/firmware"
FW_NAME="cm33.elf"

# --- Pick the right remoteproc instance based on remoteproc0/name ---
name0="$(cat "${RPROC_BASE}/remoteproc0/name" 2>/dev/null || true)"

case "$name0" in
    m33)
        RPROC="remoteproc0"
        ;;
    m0)
        RPROC="remoteproc1"
        ;;
    imx-rproc)
        RPROC="remoteproc0"
        ;;
    *)
        echo "Unexpected remoteproc0 name: '$name0' (expected 'm33' or 'm0')" >&2
        exit 1
        ;;
esac

RPROC_PATH="${RPROC_BASE}/${RPROC}"
echo "Using ${RPROC} (remoteproc0/name = ${name0})"

# --- Stop the coprocessor ---
echo "Stopping ${RPROC}..."
echo stop > "${RPROC_PATH}/state"

# --- Remove the old firmware ---
if [ -f "${FW_DIR}/${FW_NAME}" ]; then
    echo "Removing old ${FW_DIR}/${FW_NAME}..."
    rm -f "${FW_DIR}/${FW_NAME}"
else
    echo "No existing ${FW_DIR}/${FW_NAME} found, continuing."
fi

# --- Wait for the user to signal that the new firmware has been copied in ---
echo ""
echo "Waiting for new firmware to be scp'd to ${FW_DIR}/${FW_NAME}"
echo "Press ENTER once the upload is complete to continue..."
# read from the controlling terminal explicitly, in case stdin is redirected
read -r _ < /dev/tty

if [ ! -f "${FW_DIR}/${FW_NAME}" ]; then
    echo "Warning: ${FW_DIR}/${FW_NAME} still not found. Continuing anyway." >&2
fi

# --- Point remoteproc at the firmware and start it ---
echo "Setting firmware to ${FW_NAME}..."
echo "${FW_NAME}" > "${RPROC_PATH}/firmware"

echo "Starting ${RPROC}..."
echo start > "${RPROC_PATH}/state"

echo "Done. ${RPROC} state: $(cat "${RPROC_PATH}/state")"