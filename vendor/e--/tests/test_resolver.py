"""Tests for the {{ }} slot resolver — NO real API calls (fake client +
temp cache paths only)."""

import json
import os
import sys
import tempfile
import unittest
from types import SimpleNamespace
from unittest import mock

_REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
sys.path.insert(0, os.path.join(_REPO_ROOT, "src"))

from e_minus_minus.resolver import make_anthropic_resolver  # noqa: E402
from e_minus_minus.transpiler import transpile  # noqa: E402
from e_minus_minus.errors import EmmResolveError  # noqa: E402

PRIMES_PHRASE = "the first five prime numbers, as a Python list"


class FakeClient:
    """Mimics the SDK call site: client.messages.create(...).content[0].text"""

    def __init__(self, text):
        self._text = text
        self.called = False
        self.messages = self

    def create(self, **kwargs):
        self.called = True
        return SimpleNamespace(
            content=[SimpleNamespace(text=self._text)])


class RaisingClient:
    def __init__(self):
        self.messages = self

    def create(self, **kwargs):
        raise AssertionError("client must not be called on a cache hit")


class _FakeAuthenticationError(Exception):
    """Stands in for anthropic.AuthenticationError (matched by class name)."""

    def __init__(self):
        super().__init__("Error code: 401 - invalid x-api-key")


class AuthErrorClient:
    def __init__(self):
        self.messages = self

    def create(self, **kwargs):
        raise _FakeAuthenticationError()


class GenericApiErrorClient:
    def __init__(self):
        self.messages = self

    def create(self, **kwargs):
        raise RuntimeError("connection reset")


class TestResolver(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.cache_path = os.path.join(self.tmp.name, ".emm_cache.json")

    def tearDown(self):
        self.tmp.cleanup()

    def _write_cache(self, obj):
        with open(self.cache_path, "w", encoding="utf-8") as fh:
            json.dump(obj, fh)

    def test_cache_hit_no_client_call(self):
        self._write_cache({PRIMES_PHRASE: "[2, 3, 5, 7, 11]"})
        client = RaisingClient()
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        self.assertEqual(resolve(PRIMES_PHRASE), "[2, 3, 5, 7, 11]")

    def test_cache_miss_resolves_and_writes(self):
        client = FakeClient("[2, 3, 5, 7, 11]")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        self.assertEqual(resolve(PRIMES_PHRASE), "[2, 3, 5, 7, 11]")
        self.assertTrue(client.called)
        with open(self.cache_path, "r", encoding="utf-8") as fh:
            on_disk = json.load(fh)
        # v0.2.0 cache format includes a position prefix. Bare-text keys
        # (v0.1.0 format) are still HIT on read (via legacy fallback) but
        # cache writes always use the new prefixed key.
        self.assertEqual(
            on_disk[f"expression:{PRIMES_PHRASE}"], "[2, 3, 5, 7, 11]")

    def test_invalid_model_output_raises(self):
        client = FakeClient("this is not python !!!")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        with self.assertRaises(EmmResolveError):
            resolve(PRIMES_PHRASE)

    def test_code_fence_stripping(self):
        client = FakeClient("```python\n7\n```")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        self.assertEqual(resolve("a prime"), "7")

    def test_missing_key_raises_no_network(self):
        # No client, no API key, cache miss -> EmmResolveError before any
        # client construction or network.
        env = dict(os.environ)
        env.pop("ANTHROPIC_API_KEY", None)
        resolve = make_anthropic_resolver(cache_path=self.cache_path)
        with mock.patch.dict(os.environ, env, clear=True):
            with self.assertRaises(EmmResolveError) as ctx:
                resolve(PRIMES_PHRASE)
        self.assertIn("ANTHROPIC_API_KEY", str(ctx.exception))

    def test_auth_error_wrapped(self):
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=AuthErrorClient())
        with self.assertRaises(EmmResolveError) as ctx:
            resolve(PRIMES_PHRASE)
        self.assertIn("authentication", str(ctx.exception).lower())

    def test_generic_api_error_wrapped(self):
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=GenericApiErrorClient())
        with self.assertRaises(EmmResolveError) as ctx:
            resolve(PRIMES_PHRASE)
        self.assertIn("connection reset", str(ctx.exception))

    def test_end_to_end_transpile_with_cache(self):
        self._write_cache({PRIMES_PHRASE: "[2, 3, 5, 7, 11]"})
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=RaisingClient())
        with open(os.path.join(_REPO_ROOT, "examples", "primes.emm"),
                  "r", encoding="utf-8") as fh:
            src = fh.read()
        expected = "for p in [2, 3, 5, 7, 11]:\n    print(p)"
        self.assertEqual(transpile(src, resolve_slot=resolve), expected)


# --- v0.2.0 code slots ----------------------------------------------------

class TestPositionParam(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile(
            mode="w", suffix=".json", delete=False)
        self.tmp.close()
        os.unlink(self.tmp.name)
        self.cache_path = self.tmp.name

    def tearDown(self):
        if os.path.exists(self.cache_path):
            os.unlink(self.cache_path)

    def _write_cache(self, mapping):
        with open(self.cache_path, "w", encoding="utf-8") as fh:
            json.dump(mapping, fh)

    def test_v01_bare_key_still_hits_for_expression_position(self):
        # A cache written by v0.1.0 uses the bare-text key. v0.2.0 must
        # still HIT that entry for expression-position slots (back-compat).
        self._write_cache({PRIMES_PHRASE: "[2, 3, 5, 7, 11]"})
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=RaisingClient())
        self.assertEqual(
            resolve(PRIMES_PHRASE, position="expression"),
            "[2, 3, 5, 7, 11]",
        )

    def test_v02_position_scoped_cache_key_written_on_miss(self):
        # A cache miss populates a POSITION-SCOPED key, never the bare key.
        client = FakeClient("counter = 0")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        result = resolve("assign counter to zero", position="statement")
        self.assertEqual(result, "counter = 0")
        with open(self.cache_path, "r", encoding="utf-8") as fh:
            on_disk = json.load(fh)
        self.assertIn("statement:assign counter to zero", on_disk)
        self.assertEqual(
            on_disk["statement:assign counter to zero"], "counter = 0")

    def test_expression_and_statement_slots_do_not_collide(self):
        # Same slot TEXT at different positions must NOT share a cache
        # entry. v0.1.0's bare-key scheme would have collided.
        self._write_cache({
            "expression:same text": "42",
            "statement:same text": "x = 42",
        })
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=RaisingClient())
        self.assertEqual(resolve("same text", position="expression"), "42")
        self.assertEqual(resolve("same text", position="statement"), "x = 42")

    def test_statement_slot_rejects_invalid_python(self):
        client = FakeClient("this !!! is not python")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        with self.assertRaises(EmmResolveError):
            resolve("assign counter to zero", position="statement")

    def test_statement_slot_accepts_multi_statement_output(self):
        client = FakeClient('print("hello")\nprint("world")')
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        result = resolve("print two greetings", position="statement")
        self.assertEqual(result, 'print("hello")\nprint("world")')

    def test_default_position_is_expression(self):
        # No position kwarg -> expression semantics (v0.1.0 back-compat).
        client = FakeClient("7")
        resolve = make_anthropic_resolver(
            cache_path=self.cache_path, client=client)
        self.assertEqual(resolve("a prime"), "7")


if __name__ == "__main__":
    unittest.main()
