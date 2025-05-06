#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.
set -x # Print commands and their arguments as they are executed.

echo "Entrypoint wrapper (v4): Initializing..."
echo "Current user at script start: $(id -u -n) ($(id -u))"
# Arguments passed to this wrapper script are intended for the final postgres server command.
FINAL_POSTGRES_ARGS=("$@")
echo "Final postgres server args captured: ${FINAL_POSTGRES_ARGS[@]}"

if [ "$(id -u)" != '0' ]; then
    echo "ERROR: This wrapper script (entrypoint-wrapper.sh) is expected to start as root."
    exit 1
fi

CERT_MOUNT_SRC_DIR="/tmp/certs"                 # Where certs are mounted from host
CERT_FINAL_DEST_DIR="/etc/postgresql-custom-certs" # New permanent location for certs

echo "--- Ensuring certificate destination directory ${CERT_FINAL_DEST_DIR} exists ---"
mkdir -p "${CERT_FINAL_DEST_DIR}"

echo "--- Copying certificates from ${CERT_MOUNT_SRC_DIR} to ${CERT_FINAL_DEST_DIR} ---"
if [ -f "${CERT_MOUNT_SRC_DIR}/server.crt" ] && \
   [ -f "${CERT_MOUNT_SRC_DIR}/server.key" ] && \
   [ -f "${CERT_MOUNT_SRC_DIR}/ca.crt" ]; then
    echo "Source certificates found."
    cp "${CERT_MOUNT_SRC_DIR}/server.crt" "${CERT_FINAL_DEST_DIR}/server.crt"
    cp "${CERT_MOUNT_SRC_DIR}/server.key" "${CERT_FINAL_DEST_DIR}/server.key"
    cp "${CERT_MOUNT_SRC_DIR}/ca.crt" "${CERT_FINAL_DEST_DIR}/ca.crt"

    echo "Setting ownership and permissions for ${CERT_FINAL_DEST_DIR} and its contents..."
    chown -R postgres:postgres "${CERT_FINAL_DEST_DIR}"
    chmod 0700 "${CERT_FINAL_DEST_DIR}" # Directory readable/executable only by owner
    chmod 0600 "${CERT_FINAL_DEST_DIR}/server.key" # Private key only readable by owner
    chmod 0644 "${CERT_FINAL_DEST_DIR}/server.crt" # Public cert can be group/world readable
    chmod 0644 "${CERT_FINAL_DEST_DIR}/ca.crt"     # CA cert can be group/world readable

    echo "Certificates copied and permissions set in ${CERT_FINAL_DEST_DIR}."
    echo "--- Listing ${CERT_FINAL_DEST_DIR}/ ---"
    ls -la "${CERT_FINAL_DEST_DIR}"
else
    echo "ERROR: One or more source certificate files were not found or were not regular files in ${CERT_MOUNT_SRC_DIR}."
    # (Include the specific file checks here if you want from previous version)
    exit 1
fi

# PGDATA directory is still default /var/lib/postgresql/data
# It will be initialized by the main entrypoint if empty.
echo "Re-invoking original /usr/local/bin/docker-entrypoint.sh as 'postgres' user with arguments: ${FINAL_POSTGRES_ARGS[@]}"
exec gosu postgres /usr/local/bin/docker-entrypoint.sh "${FINAL_POSTGRES_ARGS[@]}"
