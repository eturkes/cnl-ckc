#!/bin/sh
# serve_tests' census stand-in: records its argv in <root>/census-argv (root = $2, after
# `--root`), touches <root>/census-touch's dd block size when that file exists (the census's
# own peak: wait4 reports a waited child's maximum), then exits with the code in <root>/census-rc.
printf '%s\n' "$@" >"$2/census-argv"
if [ -f "$2/census-touch" ]; then
    dd if=/dev/zero of=/dev/null bs="$(cat "$2/census-touch")" count=1 2>/dev/null
fi
exit "$(cat "$2/census-rc")"
