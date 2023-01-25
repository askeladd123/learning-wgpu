import shutil
from glob import glob
from subprocess import run
from sys import platform, exit, stderr, argv


def formatted(msg, msg_ansi, important, less_important=None, least_important=None):
    out = f"{msg_ansi}{msg}: \x1b[0m"

    if less_important is None:
        return f"{out}{important}"
    else:
        out = f"{out}\x1b[1m{important}\x1b[0m{less_important}"
        if least_important is not None:
            out = f"{out}\n\t{least_important}"
        return out


def warn(important, less_important=None, least_important=None):
    print(
        formatted("warning", "\x1b[33m", important, less_important, least_important),
        file=stderr
    )


def error(important, less_important=None, least_important=None):
    print(
        formatted("error", "\x1b[31m", important, less_important, least_important),
        file=stderr
    )
    exit(1)


def ok(important, less_important=None, least_important=None):
    print(
        formatted("success", "\x1b[32m", important, less_important, least_important)
    )


release = False
intro = "this is a script for compiling and bundling wasm for a static website"

if 1 < len(argv):
    if argv[1] in ["-h", "--help"]:
        print(
            f"{intro} - showing help\noptions:\n"
            "\t-r\t--release\tuse rust compiler in release mode instead of debug"
            "\n"
        )
        exit(0)
    elif argv[1] in ["-r", "--release"]:
        release = True
    else:
        error(f"{argv[1]} is not a valid argument")

print(intro)

platforms = ["win32", "linux"]
if platform not in platforms:
    warn(
        f"script not tested for {platform}, ",
        f"supported platforms: {platforms}",
        "if it fails, try building manually with cargo, wasm-bindgen cli and moving index.html, see README.md"
    )

print("checking for rust toolkit")  # ---

cmd = run(["rustup", "--version"], capture_output=True)
if cmd.returncode:
    error("couldn't find rustup, ", "download from rust-lang.org/tools/install", cmd.stderr.decode())

cmd = run(["cargo", "--version"], capture_output=True)
if cmd.returncode:
    error("couldn't find cargo", " but found rustup, idk how", cmd.stderr.decode())

print("checking for rust wasm compiler")  # ---

cmd = run(["rustup", "target", "add", "wasm32-unknown-unknown"])
if cmd.returncode:
    exit(cmd.returncode)

print("checking for wasm-bindgen cli")  # ---

cmd = run(["cargo", "install", "wasm-bindgen-cli"])
if cmd.returncode:
    exit(cmd.returncode)

print("compiling wasm")  # ---

cmd = ["cargo", "build", "--target", "wasm32-unknown-unknown"]
if release:
    cmd.append("--release")
cmd = run(cmd)

if cmd.returncode:
    exit(cmd.returncode)

print("bundling wasm")  # ---

try:
    paths = glob("target/wasm32-unknown-unknown/debug/*.wasm")
except Exception as e:
    error("no wasm file ", "in expected directory, see glob error: ", e)

cmd = run(["wasm-bindgen", "--target", "web", "--out-dir", "dist", "--no-typescript", paths[0]])

if cmd.returncode:
    exit(cmd.returncode)

print("copying index.html")  # ---
try:
    shutil.copy("index.html", "dist")
except Exception as e:
    error(
        "couldn't copy file, ",
        "missing index.html? using unsupported OS?",
        e
    )

ok("files built, ", "now you can use them with a http server")
