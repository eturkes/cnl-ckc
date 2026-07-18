"""Transpile-time `{{ }}` value-slot resolver (spec §4.4.1–§4.4.2).

`make_anthropic_resolver(...)` returns a `resolve(text) -> str` callable that:

- returns a cached Python-expression string on a cache hit (no model call);
- on a cache miss, asks an Anthropic model (Haiku by default) for a single
  Python expression, validates it parses with ``ast.parse(mode="eval")``
  (but NEVER executes it), writes it to a committed JSON cache, and returns it.

The resolver makes a model call only on a cache miss, and constructs the
Anthropic client lazily — so cache-only and mock paths need neither the
`anthropic` package nor an API key.
"""

from __future__ import annotations

import ast
import json
import os

from .errors import EmmResolveError

_DEFAULT_MODEL = "claude-haiku-4-5-20251001"

_SYSTEM_PROMPT = (
    "Translate the English description into a single Python expression that "
    "evaluates to that value. Output ONLY the Python expression on one line "
    "— no prose, no markdown, no code fences."
)

# v0.2.0 — statement-position slot prompt. Emits one or more Python statements
# that will be spliced at the caller's indentation level. Author is responsible
# for using names that fit the surrounding code; the LLM only sees the slot
# text, not the surrounding program.
_STATEMENT_SYSTEM_PROMPT = (
    "Translate the English description into one or more Python statements "
    "that accomplish it. If the description implies a variable binding, "
    "use natural Python variable names. Output ONLY the Python statements — "
    "no prose, no markdown, no code fences. Do not indent — statements will "
    "be inserted at the caller's block indentation level."
)


def _cache_key(text: str, position: str) -> str:
    """Cache key that includes position so the same slot text at expression
    vs. statement position doesn't collide. v0.1.0 stored bare `text`; on
    read we treat bare keys as expression-position for back-compat."""
    return f"{position}:{text}"


def _load_cache(cache_path: str) -> dict:
    if not os.path.exists(cache_path):
        return {}
    with open(cache_path, "r", encoding="utf-8") as fh:
        data = fh.read().strip()
    if not data:
        return {}
    return json.loads(data)


def _write_cache(cache_path: str, cache: dict) -> None:
    with open(cache_path, "w", encoding="utf-8") as fh:
        json.dump(cache, fh, sort_keys=True, indent=2)
        fh.write("\n")


def _strip_fences(text: str) -> str:
    """Defensively remove markdown code fences and surrounding whitespace."""
    s = text.strip()
    if s.startswith("```"):
        lines = s.splitlines()
        if lines and lines[0].startswith("```"):
            lines = lines[1:]
        if lines and lines[-1].strip().startswith("```"):
            lines = lines[:-1]
        s = "\n".join(lines).strip()
    return s


def make_anthropic_resolver(cache_path: str = ".emm_cache.json",
                            model: str = _DEFAULT_MODEL,
                            client=None):
    """Build a `resolve(text) -> str` slot resolver.

    `client` may be injected (e.g. a fake in tests, or a preconstructed
    Anthropic client); if None, a real client is constructed lazily on the
    first cache miss, requiring ``ANTHROPIC_API_KEY``.
    """
    state = {"client": client}

    def resolve(text: str, position: str = "expression") -> str:
        cache = _load_cache(cache_path)
        # v0.2.0 — position-scoped cache key. Fall back to the v0.1.0 bare-key
        # entry for `position="expression"` so existing caches keep hitting.
        key = _cache_key(text, position)
        if key in cache:
            return cache[key]
        if position == "expression" and text in cache:
            legacy = cache[text]
            # Migrate legacy entry to the new key format on next write.
            cache[key] = legacy
            _write_cache(cache_path, cache)
            return legacy

        c = state["client"]
        if c is None:
            api_key = os.environ.get("ANTHROPIC_API_KEY")
            if not api_key:
                raise EmmResolveError(
                    "set ANTHROPIC_API_KEY to resolve {{ }} slots "
                    f"(no cached value for {position} slot: {text!r})")
            try:
                import anthropic  # lazy: only needed on a live cache miss
            except ImportError as exc:
                raise EmmResolveError(
                    "the 'anthropic' package is required to resolve {{ }} "
                    "slots but is not installed. Run: "
                    "pip install 'e-minus-minus[llm]'") from exc
            c = anthropic.Anthropic(api_key=api_key)
            state["client"] = c

        system = (
            _STATEMENT_SYSTEM_PROMPT if position == "statement"
            else _SYSTEM_PROMPT
        )

        try:
            response = c.messages.create(
                model=model,
                # Statement slots may emit multiple lines; give more room.
                max_tokens=1024 if position == "statement" else 256,
                temperature=0,
                system=system,
                messages=[{"role": "user", "content": text}],
            )
        except Exception as exc:  # SDK / transport / auth errors
            name = type(exc).__name__
            if "Authentication" in name or "invalid x-api-key" in str(exc):
                raise EmmResolveError(
                    f"Anthropic rejected the API key (authentication error) "
                    f"while resolving {position} slot {text!r}. Check that "
                    f"ANTHROPIC_API_KEY is a valid, current key with no extra "
                    f"quotes or whitespace. Original error: {exc}") from exc
            raise EmmResolveError(
                f"LLM call failed while resolving {position} slot {text!r}: "
                f"{name}: {exc}") from exc
        raw = response.content[0].text
        emitted = _strip_fences(raw)

        # Validate the model's output parses as Python. Expression slots must
        # parse in `eval` mode (single expression); statement slots must parse
        # in `exec` mode (one or more statements).
        try:
            ast.parse(emitted,
                      mode="exec" if position == "statement" else "eval")
        except SyntaxError as exc:
            raise EmmResolveError(
                f"model did not return valid Python for {position} slot "
                f"{text!r}; got: {raw!r}") from exc

        cache[key] = emitted
        _write_cache(cache_path, cache)
        return emitted

    return resolve
