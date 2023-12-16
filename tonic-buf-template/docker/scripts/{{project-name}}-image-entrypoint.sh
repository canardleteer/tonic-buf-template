#!/bin/bash

echo "{{project-name}} image started."

RUST_LOG=${RUST_LOG:=info}
SERVER_LISTEN_ADDR=${SERVER_LISTEN_ADDR:=0.0.0.0}
SERVER_LISTEN_PORT=${SERVER_LISTEN_PORT:=0.0.0.0}
TARGET_SERVER_ADDR=${TARGET_SERVER_ADDR:=${SERVER_LISTEN_ADDR}}
TARGET_SERVER_PORT=${SERVER_LISTEN_PORT:=${SERVER_LISTEN_PORT}}
USE_CLIENT_BINARY=${USE_CLIENT_BINARY:=false}

echo "-------------------------------------------------------------------------------"
echo "environment:"
echo "* RUST_LOG: ${RUST_LOG}"
echo "* SERVER_LISTEN_ADDR: ${SERVER_LISTEN_ADDR}"
echo "* SERVER_LISTEN_PORT: ${SERVER_LISTEN_PORT}"
echo "* TARGET_SERVER_ADDR: ${TARGET_SERVER_ADDR}"
echo "* TARGET_SERVER_PORT: ${TARGET_SERVER_PORT}"
echo "* USE_CLIENT_BINARY: ${USE_CLIENT_BINARY}"
echo "-------------------------------------------------------------------------------"

# By default, run the server, but offer a client route as well:

if [ "${USE_CLIENT_BINARY}" = true ]; then
    ${CLIENT_ENTRYPOINT_BINARY} "$@"
else
    ${SERVER_ENTRYPOINT_BINARY} "$@"
fi