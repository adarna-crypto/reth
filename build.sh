#!/usr/bin/env bash
set -e
make maxperf
echo "Successfully built reth: $(target/maxperf/reth -V)"
echo "Now install it into /usr/local/bin but symlink it to retain the older version"
ls -la /usr/local/bin/reth*
