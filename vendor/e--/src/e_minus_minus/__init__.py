"""E-- (English--) — a deterministic English-to-Python transpiler.

Public API:

    from e_minus_minus import transpile
    python_source = transpile(canonical_emm_source)

For CLI usage, install the package and use the `emm-transpile` command.
"""

from .errors import (
    EmmSyntaxError,
    EmmResolveError,
    EmmNormalizeError,
)


def __getattr__(name):
    if name == "transpile":
        from .transpiler import transpile
        globals()[name] = transpile
        return transpile
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__version__ = "0.2.0"

__all__ = [
    "transpile",
    "EmmSyntaxError",
    "EmmResolveError",
    "EmmNormalizeError",
    "__version__",
]
