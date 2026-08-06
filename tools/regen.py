import os
import pathlib
import subprocess
import sys
def walk_tree(dir_path):
    entries = []
    for entry in sorted(dir_path.iterdir()):
        symlink = entry.is_symlink()
        directory = False
        if not symlink:
            directory = entry.is_dir()
        if directory:
            nested = walk_tree(entry)
            for nested_entry in nested:
                entries.append(nested_entry)
        else:
            entries.append(entry)
    return entries
def discover(roots):
    sources = []
    for root_name in roots:
        root_path = pathlib.Path(root_name)
        for source_path in walk_tree(root_path):
            if source_path.suffix == ".emm":
                sources.append(source_path.as_posix())
    return sorted(sources)
def python_path_for(source_rel):
    source_path = pathlib.Path(source_rel)
    target_path = source_path.with_suffix(".py")
    return target_path.as_posix()
def generated_paths_for(sources):
    generated = []
    for source_rel in sources:
        generated.append(python_path_for(source_rel))
    return sorted(generated)
def compile_one(source_rel, exe, env):
    res = subprocess.run([exe, "-P", "-m", "e_minus_minus.strict", source_rel], capture_output=True, env=env)
    return res
def orphan_python_paths(roots):
    orphans = []
    for root_name in roots:
        root_path = pathlib.Path(root_name)
        for candidate in walk_tree(root_path):
            if candidate.suffix == ".py":
                source_path = candidate.with_suffix(".emm")
                if not source_path.is_file():
                    orphans.append(candidate.as_posix())
    return sorted(orphans)
def unauthorized_python_paths(generated):
    git_res = subprocess.run(["git", "ls-files", "--", "*.py"], capture_output=True, check=True)
    tracked_text = git_res.stdout.decode("utf-8")
    unauthorized = []
    for tracked in sorted(tracked_text.splitlines()):
        vendored = tracked.startswith("vendor/e--/src/")
        generated_member = tracked in generated
        if not vendored:
            if not generated_member:
                unauthorized.append(tracked)
    return unauthorized
def report_paths(category, paths):
    for path in sorted(paths):
        print("regen: " + category + ": " + path)
def check_mode(roots, sources, exe, env):
    compile_errors = []
    missing = []
    drift = []
    for source_rel in sources:
        res = compile_one(source_rel, exe, env)
        if res.returncode != 0:
            compile_errors.append(source_rel)
        else:
            target_rel = python_path_for(source_rel)
            target_path = pathlib.Path(target_rel)
            if not target_path.is_file():
                missing.append(target_rel)
            else:
                current_bytes = target_path.read_bytes()
                if current_bytes != res.stdout:
                    drift.append(target_rel)
    generated = generated_paths_for(sources)
    orphans = orphan_python_paths(roots)
    unauthorized = unauthorized_python_paths(generated)
    report_paths("compile-error", compile_errors)
    report_paths("missing", missing)
    report_paths("drift", drift)
    report_paths("orphan", orphans)
    report_paths("unauthorized", unauthorized)
    violations = len(compile_errors)
    violations = violations + len(missing)
    violations = violations + len(drift)
    violations = violations + len(orphans)
    violations = violations + len(unauthorized)
    if violations > 0:
        print("regen: violations: " + str(violations))
        raise SystemExit(1)
    print("regen: check ok")
def regenerate_mode(sources, exe, env):
    compile_errors = []
    staged = {}
    for source_rel in sources:
        res = compile_one(source_rel, exe, env)
        if res.returncode != 0:
            compile_errors.append(source_rel)
        else:
            target_rel = python_path_for(source_rel)
            staged.update({target_rel: res.stdout})
    report_paths("compile-error", compile_errors)
    violations = len(compile_errors)
    if violations > 0:
        print("regen: violations: " + str(violations))
        raise SystemExit(1)
    pid_text = str(os.getpid())
    for target_rel in sorted(staged):
        payload = staged.pop(target_rel)
        temp_rel = target_rel + ".tmp." + pid_text
        temp_path = pathlib.Path(temp_rel)
        temp_path.write_bytes(payload)
        os.replace(temp_rel, target_rel)
        print("regen: wrote " + target_rel)
    print("regen: regenerate ok")
roots = ["tools"]
strict_src = pathlib.Path("vendor/e--/src/e_minus_minus/strict.py")
if not (strict_src.is_file()):
    raise AssertionError("requirement failed")
argv = list(sys.argv)
argv.pop(0)
if len(argv) != 1:
    print("regen: usage: regen.py --check | --regenerate", file=sys.stderr)
    raise SystemExit(2)
mode = argv.pop(0)
exe = sys.executable
env = os.environ.copy()
env.update({"PYTHONPATH": "vendor/e--/src", "PYTHONDONTWRITEBYTECODE": "1"})
sources = discover(roots)
if mode == "--regenerate":
    regenerate_mode(sources, exe, env)
else:
    if mode == "--check":
        check_mode(roots, sources, exe, env)
    else:
        print("regen: usage: regen.py --check | --regenerate", file=sys.stderr)
        raise SystemExit(2)
