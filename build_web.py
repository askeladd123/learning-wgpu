import os
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
rm_src = False
intro = "this is a script for compiling and bundling wasm for a static website"

args = [
    ["-h", "--help"],
    ["-r", "--release"],
    ["-c", "--clean"],
    ["--rm-src"]
]

if 1 < len(argv):

    found = False

    if not set(argv[1:])&set(sum(args, [])):
        error(f"{argv[1:]} is not valid")

    if set(argv[1:])&set(args[0]):
        print(
            f"{intro} - showing help\noptions:\n"
            "\t-r\t--release\tuse rust compiler in release mode instead of debug\n"
            "\t-c\t--clean\tremove files from last build\n"
            "\t\t--rm-src\tremoves all development files!!!"
        )
        exit(0)
    if set(argv[1:])&set(args[3]):
        print("are you sure you want to remove all development files? they cannot be recovered\n"
              "\ty=YES, n=NO")
        i = input()
        if i.lower() not in ["y", "n"]:
            error(f"{i} is not valid, ", "try y for YES, n for NO")
        i = i.lower()
        if i == "n":
            exit(0)
        if i == "y":
            rm_src = True

    release = bool(set(argv[1:])&set(args[1]))
    clean = bool(set(argv[1:])&set(args[2]))

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

# print("copying index.html")  # ---
# try:
#     shutil.copy("index.html", "dist")
# except Exception as e:
#     error(
#         "couldn't copy file, ",
#         "missing index.html? using unsupported OS?",
#         e
#     )

if rm_src:
    print("removing development files")
    dev_folders = next(os.walk("."))[1]
    dev_files = next(os.walk("."))[2]

    dev_folders.remove("dist")
    dev_folders.remove(".git")
    dev_files.remove("index.html")

    try:
        for f in dev_folders:
            shutil.rmtree(f)
        for f in dev_files:
            os.remove(f)
    except Exception as e:
            warn("couldn't remove all development files", "see shutil/os error", e)


ok("files built, ", "now you can use them with a http server")
