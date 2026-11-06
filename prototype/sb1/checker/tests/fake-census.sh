#!/bin/sh
# serve_tests' census stand-in: records its argv in <root>/census-argv (root = $2, after
# `--root`), then exits with the code written in <root>/census-rc.
printf '%s\n' "$@" >"$2/census-argv"
exit "$(cat "$2/census-rc")"
