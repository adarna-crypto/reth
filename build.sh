#!/usr/bin/env bash
set -e
# make maxperf
VERSION_OUTPUT=$(target/maxperf/reth -V)

echo "Successfully built reth: $VERSION_OUTPUT"

VERSION=$(echo "$VERSION_OUTPUT" | cut -d' ' -f2 | cut -d'-' -f1)
COMMIT=$(echo "$VERSION_OUTPUT" | tr -d '()' | awk '{print $NF}')

echo "Version: $VERSION"
echo "Commit: $COMMIT"

INSTALL_PATH="/usr/local/bin/reth-$VERSION-$COMMIT"
sudo cp target/maxperf/reth $INSTALL_PATH
sudo systemctl stop reth
sudo ln -sf $INSTALL_PATH /usr/local/bin/reth

ls -la /usr/local/bin/reth*