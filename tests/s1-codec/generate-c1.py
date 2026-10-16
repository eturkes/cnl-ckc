#!/usr/bin/env python3
"""Frozen C1 oracle inputs for s1_codec.rs; S1 expectations = D1/D3/D4 in that test.

Run with SWI 9.2.9 on PATH:
  python3 -I tests/s1-codec/generate-c1.py <C1-checkout-at-d972a813> <output-directory>
Output is independent of the S1 compiler. These c1.pl.in snapshots keep 15 pairs;
schema migration applies to the test's expected_pl model, not these oracle bytes.
"""
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ULEX = (
    "noun_sg('strong-grade','strong-grade',neutr).\n"
    "noun_sg('low-grade','low-grade',neutr).\n"
    "adj_itr(preferred,preferred).\n"
    "prep('instead-of','instead-of').\n"
)
C1_TABLE = '# format: kind\tlemma\tvalue\nunit\tunused-hour\thour\n'
S1_TABLE = (
    '# format: kind\tlemma\tvalue\n'
    'strength\tstrong-grade\tstrong\n'
    'evidence\tlow-grade\tlow\n'
    'preference\tpreferred\tpreferred\n'
    'alternative\tinstead-of\talternative\n'
)
CASES = {
    'certdoc': ('doc',
        'A recommendation is a strong-grade and is a low-grade.\n'
        'A medicine is preferred.\n'
        'A treatment is preferred instead-of a procedure.\n'
        'A doctor uses a medicine instead-of a treatment.\n'
        'A medicine should not be preferred instead-of a treatment.\n'),
    'bare': ('doc', 'A medicine is preferred.\n'),
    'combined': ('doc', 'A medicine is preferred instead-of a treatment.\n'),
    'pref-query': ('query', 'Which medicine is preferred?\n'),
    'alternative-query': ('query', 'Does a medicine precede a treatment instead-of a procedure?\n'),
    'grade-query': ('query', 'Is a recommendation a strong-grade?\n'),
}


def run(args, source=''):
    result = subprocess.run(args, input=source, text=True, capture_output=True, timeout=30)
    if result.returncode or result.stderr:
        raise RuntimeError(f'{args}: rc={result.returncode}; stderr={result.stderr}')
    return result.stdout


def generate(base, out):
    version = run(['swipl', '--version'])
    assert version.startswith('SWI-Prolog version 9.2.9 '), version
    scratch = out / '.generation'
    scratch.mkdir(parents=True, exist_ok=True)
    try:
        with tempfile.TemporaryDirectory(dir=scratch) as directory:
            temporary = Path(directory)
            stage = temporary / 'ape'
            shutil.copytree(base / 'vendor/ape', stage)
            shutil.copyfile(base / 'vendor/clex/clex_lexicon.pl', stage / 'prolog/lexicon/clex_lexicon.pl')
            parser = str(stage / 'prolog/parser').replace("'", "\\'")
            staged = subprocess.run(['swipl', '-O', '-f', 'none', '-F', 'none', '-g',
                f"working_directory(_, '{parser}'), [fit_to_plp], halt.", '-t', 'halt'],
                text=True, capture_output=True, timeout=30)
            assert staged.returncode == 0, staged.stderr
            prefix = ['swipl', '-q', '-f', 'none', '-F', 'none', '-s',
                str(stage / 'prolog/ace_to_pl.pl'), '-g', 'main', '-t', 'halt(9)', '--']
            preflight = run(prefix + [str(stage), 'control', '-', '-'], 'A patient exists.\n')
            assert preflight.count(':- multifile(') == 15, 'C1 compiler required (15 pairs)'
            table, ulex = temporary / 'vocabulary.tsv', temporary / 'lexicon.ulex'
            table.write_text(C1_TABLE)
            ulex.write_text(ULEX)
            files = {'lexicon.ulex.in': ULEX, 'c1-vocabulary.tsv.in': C1_TABLE,
                'vocabulary.tsv.in': S1_TABLE}
            for name, (mode, ace) in CASES.items():
                files[f'{name}/ace.txt.in'] = ace
                args = prefix + (['question'] if mode == 'query' else [])
                args += [str(stage), name, str(table), str(ulex)]
                files[f'{name}/c1.pl.in'] = run(args, ace)
                if mode == 'doc':
                    files[f'{name}/c1-proof.txt.in'] = run(args + ['proof'], ace)
                files[f'{name}/dump.txt.in'] = run(['swipl', '-q', '-f', 'none', '-F', 'none', '-s',
                    str(base / 'rust/ckc/prolog/drs_dump.pl'), '-g', 'main', '-t', 'halt(9)', '--',
                    str(stage), str(ulex)], ace)
            for name, text in files.items():
                target = out / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(text)
            print(f'C1 oracle: {len(files)} files; 6 ACE/DRS/projection cases')
    finally:
        scratch.rmdir()


if __name__ == '__main__':
    if len(sys.argv) != 3:
        raise SystemExit('usage: generate-c1.py <C1-checkout> <output-directory>')
    generate(Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve())
