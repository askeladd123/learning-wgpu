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
clean = False
intro = "this is a script for compiling and bundling wasm for a static website"

args = [
    ["-h", "--help"],
    ["-r", "--release"],
    ["-c", "--clean"],
]

if 1 < len(argv):

    found = False

    if not set(argv[1:])&set(sum(args, [])):
        error(f"{argv[1:]} is not valid")

    # if argv[1] in ["-h", "--help"]:
    if set(argv[1:])&set(args[0]):
        print(
            f"{intro} - showing help\noptions:\n"
            "\t-r\t--release\tuse rust compiler in release mode instead of debug\n"
            "\t-c\t--clean\tremove files from last build\n"
        )
        exit(0)
    release = bool(set(argv[1:])&set(args[1]))
    clean = bool(set(argv[1:])&set(args[2]))

# elif argv[1] in ["-r", "--release"]:
    #     release = True
    # else:
    #     error(f"{argv[1:]} is not valid")

print(intro)

platforms = ["win32", "linux"]
if platform not in platforms:
    warn(
        f"script not tested for {platform}, ",
        f"supported platforms: {platforms}",
        "if it fails, try building manually with cargo, wasm-bindgen cli and moving index.html, see README.md"
    )

print("checking for rust toolkit")  # ---

cmd = run(["rustup", "--version"], capture_output=True, text=True)
if cmd.returncode:
    error("couldn't find rustup, ", "download from rust-lang.org/tools/install", cmd.stderr)

cmd = run(["cargo", "--version"], capture_output=True, text=True)
if cmd.returncode:
    error("couldn't find cargo", " but found rustup, idk how", cmd.stderr)

print("checking for rust wasm compiler")  # ---

cmd = run(["rustup", "target", "add", "wasm32-unknown-unknown"])
if cmd.returncode:
    exit(cmd.returncode)

print("checking for wasm-bindgen cli")  # ---

cmd = run(["cargo", "install", "wasm-bindgen-cli"])
if cmd.returncode:
    exit(cmd.returncode)

if clean:
    print("removing old files")
    cmd = run(["cargo", "clean"], capture_output=True, text=True)
    if cmd.returncode:
        warn("failed to clean, ", "see cargo error:", cmd.stdout)
    try:
        shutil.rmtree("dist")
    except Exception as e:
        warn("failed to clean, ", "couldn't delete dist folder, see shutil error:", e)

print("compiling wasm")  # ---

cmd = ["cargo", "build", "--lib", "--target", "wasm32-unknown-unknown"]
if release:
    cmd.append("--release")
cmd = run(cmd)

if cmd.returncode:
    exit(cmd.returncode)

print("bundling wasm")  # ---

path = "target/wasm32-unknown-unknown/"
if release:
    path += "release/*.wasm"
else:
    path += "debug/*.wasm"

try:
    paths = glob(path)
except Exception as e:
    error("no wasm file ", "in expected directory, see glob error: ", e)

if not paths:
    error("no wasm file ", "in expected directory", f"missing {path}")

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
