# just --dump → one bash script per recipe in DIR (shebang bodies as written, line recipes
# under the justfile's `bash -euo pipefail`); {{ … }} interpolations → the word __just__.
function flush() {
    if (name == "") return
    f = DIR "/" name ".sh"
    if (!shebang) { print "#!/usr/bin/env bash" > f; print "set -euo pipefail" > f }
    for (i = 1; i <= n; i++) print body[i] > f
    close(f); name = ""; n = 0
}
/^[A-Za-z_][A-Za-z0-9_-]*( [^:]*)?:/ && !/^[A-Z_]+ :=/ { flush(); name = $1; sub(/:.*/, "", name); n = 0; shebang = 0; next }
/^    / && name != "" {
    line = substr($0, 5)
    gsub(/\{\{[^}]*\}\}/, "__just__", line)
    if (n == 0 && line ~ /^#!/) shebang = 1
    if (!shebang) sub(/^[@-]+/, "", line)
    body[++n] = line; next
}
/^[^ ]/ { flush() }
END { flush() }
