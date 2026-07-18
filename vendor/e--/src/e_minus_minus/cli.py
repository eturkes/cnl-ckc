"""CLI entry point for the E-- transpiler package.

Installed as the `emm-transpile` console script via pyproject.toml. Simply
delegates to :func:`transpiler.main` so the existing argparse surface is
preserved verbatim — this wrapper exists to give PyPI a stable entry point
that stays put across future internal refactors.
"""

from .transpiler import main


def _entry() -> None:
    """PyPI console-script entry — wraps ``main`` so a bare exit code becomes a
    proper ``SystemExit``."""
    raise SystemExit(main())


if __name__ == "__main__":
    _entry()
