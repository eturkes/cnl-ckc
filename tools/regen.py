import argparse
import os
import pathlib
import subprocess
import sys
def discover(roots):
    sources = []
    for root_name in roots:
        root_path = pathlib.Path(root_name)
        for source_path in sorted(root_path.rglob("*.emm")):
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
def vendor_python_paths():
    manifest_path = pathlib.Path("vendor/e--/MANIFEST.sha256")
    manifest_text = manifest_path.read_text(encoding="utf-8")
    vendor_paths = []
    for line in manifest_text.splitlines():
        if line:
            relative_path = ""
            for piece in line.split("  "):
                relative_path = piece
            relative_path = relative_path.removeprefix("./")
            full_path = "vendor/e--/" + relative_path
            if full_path.endswith(".py"):
                vendor_paths.append(full_path)
    return sorted(vendor_paths)
def orphan_python_paths(roots):
    orphans = []
    for root_name in roots:
        root_path = pathlib.Path(root_name)
        for candidate in sorted(root_path.rglob("*.py")):
            source_path = candidate.with_suffix(".emm")
            if not source_path.is_file():
                orphans.append(candidate.as_posix())
    return sorted(orphans)
def unauthorized_python_paths(generated):
    allowed = vendor_python_paths() + generated
    git_res = subprocess.run(["git", "ls-files", "--", "*.py"], capture_output=True, check=True)
    tracked_text = git_res.stdout.decode("utf-8")
    unauthorized = []
    for tracked in sorted(tracked_text.splitlines()):
        if not (tracked in allowed):
            unauthorized.append(tracked)
    return unauthorized
def conftest_paths():
    repo_path = pathlib.Path(".")
    found = []
    for candidate in sorted(repo_path.rglob("conftest.py")):
        hidden = False
        for part in candidate.parts:
            if part.startswith("."):
                hidden = True
        if not hidden:
            found.append(candidate.as_posix())
    return found
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
    conftests = conftest_paths()
    report_paths("compile-error", compile_errors)
    report_paths("missing", missing)
    report_paths("drift", drift)
    report_paths("orphan", orphans)
    report_paths("unauthorized", unauthorized)
    report_paths("conftest", conftests)
    violations = len(compile_errors)
    violations = violations + len(missing)
    violations = violations + len(drift)
    violations = violations + len(orphans)
    violations = violations + len(unauthorized)
    violations = violations + len(conftests)
    if violations > 0:
        print("regen: violations: " + str(violations))
        raise SystemExit(1)
    print("regen: check ok")
def regenerate_mode(sources, exe, env):
    compile_errors = []
    for source_rel in sources:
        res = compile_one(source_rel, exe, env)
        if res.returncode != 0:
            compile_errors.append(source_rel)
    report_paths("compile-error", compile_errors)
    violations = len(compile_errors)
    if violations > 0:
        print("regen: violations: " + str(violations))
        raise SystemExit(1)
    pid_text = str(os.getpid())
    for source_rel in sources:
        res = compile_one(source_rel, exe, env)
        if not (res.returncode == 0):
            raise AssertionError("requirement failed")
        target_rel = python_path_for(source_rel)
        temp_rel = target_rel + ".tmp." + pid_text
        temp_path = pathlib.Path(temp_rel)
        temp_path.write_bytes(res.stdout)
        os.replace(temp_rel, target_rel)
        print("regen: wrote " + target_rel)
    print("regen: regenerate ok")
roots = ["tools"]
strict_src = pathlib.Path("vendor/e--/src/e_minus_minus/strict.py")
if not (strict_src.is_file()):
    raise AssertionError("requirement failed")
parser = argparse.ArgumentParser(prog="regen")
parser.add_argument("--check", action="store_true")
parser.add_argument("--regenerate", action="store_true")
args = parser.parse_args()
exe = sys.executable
env = os.environ.copy()
env.update({"PYTHONPATH": "vendor/e--/src", "PYTHONDONTWRITEBYTECODE": "1"})
sources = discover(roots)
if args.regenerate:
    regenerate_mode(sources, exe, env)
else:
    check_mode(roots, sources, exe, env)
