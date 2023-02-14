// #![allow(unused)]
use glob::glob;
use std::fmt::Display;
use std::process::{exit, Command};

fn warn(msg: impl Display) {
    println!("\x1b[33mwarning: \x1b[0m{msg}")
}

fn error(msg: impl Display) {
    eprintln!("\x1b[31merror: \x1b[0m{msg}");
    std::process::exit(1)
}

fn ok(msg: impl Display) {
    println!("\x1b[32msuccess: \x1b[0m{msg}")
}

fn main() {
    let mut release = false;
    let mut clean = false;
    let mut rm_src = false;

    let intro = "this is a script for compiling and bundling wasm for a static website";

    for arg in std::env::args().skip(1) {
        match arg.to_lowercase().as_str(){
            "-h"|"--help"=>{
                println!(
                    "{intro} - showing help\nflags:\n{:^8}{:<15}{}\n{:^8}{:<15}{}\n{:^8}{:<15}{}\n",
                    "-r", "--release", "use rust compiler in release mode instead of debug",
                    "-c", "--clean", "remove files from last build",
                    "", "--rm-dev", "removes all development files!!!"
                );
                exit(0);
            },
            "-c"|"--clean"=>{clean=true},
            "-r"|"--release"=>{release=true},
            "--rm-dev"=>{
                println!(
                    "are you sure you want to remove all development files? 
                    they cannot be recovered (yes or no)"
                );
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line = line.trim_end().into();
                match line.to_lowercase().as_str(){
                    "y"|"yes"=>{rm_src=true},
                    "n"|"no"=>{println!("ok, exiting...");exit(0)}
                    _ => {error(format!("{line} is not valid, try y for YES, n for NO"));}
                }
            },
            _=>{error(format!("{arg} is not a valid argument, pass -h or --help as argument to see possible flags"))}
        }
    }

    println!("checking for rust wasm compiler");
    if Command::new("cargo").arg("--version").output().is_err() {
        error("cargo build tool and package manager is required, please install the rust toolkit");
    }

    match Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
    {
        Err(e) => {
            error(format!(
                "rustup command didn't work, do you have rustup the rust toolchain installed?\n{e}"
            ));
        }
        Ok(out) if !out.status.success() => {
            error(format!(
                "rustup failed: \n{}",
                std::str::from_utf8(&out.stderr).unwrap()
            ));
        }
        Ok(out) => {
            if !std::str::from_utf8(&out.stdout)
                .unwrap()
                .lines()
                .any(|v| v == "wasm32-unknown-unknown")
            {
                Command::new("rustup")
                    .args(["target", "add", "wasm32-unknown-unknown"])
                    .status()
                    .unwrap();
            }
        }
    }

    println!("checking for wasm-bindgen-cli");
    if Command::new("wasm-bindgen")
        .arg("--version")
        .output()
        .is_err()
    {
        Command::new("cargo")
            .args(["install", "wasm-bindgen-cli"])
            .status()
            .unwrap();
    }
    if clean {
        println!("removing old files");
        if Command::new("cargo").arg("clean").status().is_err() {
            warn("failed to clean");
        }
        if std::fs::remove_dir_all("dist").is_err() {
            warn("failed to remove dist folder");
        }
    }

    println!("compiling wasm");

    let mut args = vec!["build", "--target", "wasm32-unknown-unknown"];
    if release {
        args.push("--release");
    }

    if Command::new("cargo").args(args).status().is_err() {
        error("failed to compile wasm");
    }

    println!("bundling wasm");
    let path = String::from("target/wasm32-unknown-unknown/")
        + if release {
            "release/*.wasm"
        } else {
            "debug/*.wasm"
        };
    let mut paths = glob(&path).unwrap();
    let path = match paths.next() {
        Some(Ok(v)) => v,
        _ => {
            error("couldn't find wasm file");
            "".into()
        }
    };
    if paths.next().is_some() {
        error("multiple wasm files, can't choose");
    }

    Command::new("wasm-bindgen")
        .arg(path)
        .args(["--target", "web", "--out-dir", "dist", "--no-typescript"])
        .status()
        .unwrap();

    if rm_src {
        println!("removing development files");

        let mut files = Vec::new();
        let mut folders = Vec::new();

        for val in std::fs::read_dir(".").unwrap() {
            let f = val.unwrap();

            let name = f.file_name().into_string().unwrap();
            if name.contains("index.html") || name.contains("dist") || name.contains(".git") {
                continue;
            }

            if f.file_type().unwrap().is_file() {
                files.push(f.file_name());
            } else if f.file_type().unwrap().is_dir() {
                folders.push(f.file_name())
            } else {
                error("weird file types");
            }
        }

        for file in files.iter() {
            match std::fs::remove_file(file) {
                Ok(_) => {}
                Err(e) => warn(format!("failed to delete file: {file:?}\n\tbecause: {e}")),
            }
        }

        for folder in folders.iter() {
            match std::fs::remove_dir_all(folder) {
                Ok(_) => {}
                Err(e) => warn(format!(
                    "failed to delete folder: {folder:?}\n\tbecause: {e}"
                )),
            }
        }
    }

    ok("files built, now you can use them with a http server")
}
