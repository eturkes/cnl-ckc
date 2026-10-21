#!/usr/bin/env python3
# Verus-issue exposure scans (verus-lang/verus #3144 #3146 #3147 #3154-#3160) over ckc-kernel/src + ckc-spec/src, each pattern with a positive control.
# usage: python3 -I .agent/archive/scans/verus-issues.py <repo-root>; rerun at every Verus bump (deferred row N17).
import re, sys, pathlib, collections

root = pathlib.Path(sys.argv[1])
files = sorted(list((root / 'rust/ckc-kernel/src').glob('*.rs')) + list((root / 'rust/ckc-spec/src').glob('*.rs')))
texts = {f: f.read_text(encoding='utf-8') for f in files}

def strip(src):
    # drop line comments, block comments and string/char literals (keeps line structure)
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i); j = n if j < 0 else j; i = j; continue
        if src.startswith('/*', i):
            j = src.find('*/', i + 2); j = n if j < 0 else j + 2
            out.append('\n' * src.count('\n', i, j)); i = j; continue
        if c == '"' or (c == 'b' and i + 1 < n and src[i + 1] == '"'):
            i += 1 if c == '"' else 2
            while i < n and src[i] != '"':
                if src[i] == '\\': i += 1
                elif src[i] == '\n': out.append('\n')
                i += 1
            out.append('""'); i += 1; continue
        if c == "'" and i + 2 < n and (src[i + 2] == "'" or (src[i + 1] == '\\' and src.find("'", i + 2) - i <= 10)):
            j = src.find("'", i + 2 if src[i + 1] == '\\' else i + 1)
            if j > 0 and j - i <= 10:
                out.append("' '"); i = j + 1; continue
        out.append(c); i += 1
    return ''.join(out)

code = {f: strip(t) for f, t in texts.items()}

patterns = [
    ('3157', r'cfg\(any\(\)\)', '#[cfg(any())] fn x() {}'),
    ('3146', r'proof_decl!', 'proof_decl! { }'),
    ('3147', r'verifier::exec\b|verifier\(exec\)', '#[verifier::exec] proof fn f() {}'),
    ('3158', r'\b(format|print|println|eprint|eprintln|write|writeln|format_args)!', 'let s = format!("{}", 1);'),
    ('3155', r'\[[^\]\[]*;\s*[A-Za-z_(]', 'let a = [0u8; core::mem::size_of_val(&N)];'),
    ('3156', r'macro_rules!\s*(panic|assert|assert_eq|assert_ne|unreachable|todo|unimplemented|debug_assert|vec|format|matches|line|file|column|stringify|concat|env|include)\b', 'macro_rules! panic { () => {} }'),
]
for issue, pat, control in patterns:
    rx = re.compile(pat)
    hits = [(f.relative_to(root / 'rust'), n + 1) for f, t in code.items() for n, l in enumerate(t.split('\n')) if rx.search(l)]
    ctl = len(rx.findall(control))
    print(f'{issue}\thits={len(hits)}\tcontrol={ctl}\tpattern={pat}' + ''.join(f'\n\t{p}:{n}' for p, n in hits[:5]))

names = collections.Counter(m.group(1) for t in code.values() for m in re.finditer(r'macro_rules!\s*(\w+)', t))
print('macro_rules names:', dict(sorted(names.items())))
item_calls = [(f.relative_to(root / 'rust'), n + 1, l.strip()) for f, t in code.items() for n, l in enumerate(t.split('\n'))
              if re.match(r'^[a-z_][a-z0-9_]*!\s*[\{\(\[]', l) and not re.match(r'^(verus|macro_rules)!', l)]
print('item-level macro calls (col 0) other than verus!/macro_rules! (#3154):', item_calls)
print('col-0 verus! blocks:', sum(len(re.findall(r'^verus!\s*\{', t, re.M)) for t in code.values()))

def nested_fns(src):
    # stack of block kinds; a `fn` keyword seen while a fn body is open = nested
    toks = re.finditer(r'\bfn\b|\{|\}|;', src)
    stack, pending, member, nested = [], None, 0, 0
    for m in toks:
        tok = m.group(0)
        if tok == 'fn':
            if 'fn' in stack: nested += 1
            elif stack: member += 1
            pending = 'fn'
        elif tok == '{':
            stack.append(pending or 'other'); pending = None
        elif tok == '}':
            if stack: stack.pop()
        elif tok == ';' and pending == 'fn':
            pending = None   # declaration without body
    return member, nested

tot = [0, 0]
for t in code.values():
    m, n = nested_fns(t)
    tot[0] += m; tot[1] += n
print(f'indented fns: member={tot[0]} nested={tot[1]}')
print('control: member=%d nested=%d (expected 1 1)' % nested_fns('impl X { fn outer() { fn inner() {} } }'))
just = (root / 'justfile').read_text() + ''.join(p.read_text() for p in (root / '.github').rglob('*.yml'))
print('3159: --verify-only-module in justfile + CI:', just.count('--verify-only-module'), '(control: the canary case passes it:',
      (root / 'rust/verus-canary/verify_only_module_partial.rs').read_text().count('--verify-only-module'), ')')
allow = (root / 'rust/trust/escape-allowlist.tsv').read_text().split('\n')
rows = [r for r in allow if '\tassume' in r or r.endswith('\tassume')]
print('3160: escape-allowlist rows naming assume:', len(rows), [r.split('\t')[0] for r in rows])
