use std::fmt::Display;


fn formatted(msg_type:impl Display, msg_ansi:&str, msg:impl Display)->impl Display{
    format!("{msg_ansi}{msg_type}: \x1b[0m{msg}")
}

fn warn(msg: impl Display){
    println!("{}", formatted("warning", "\x1b[33m", msg))
}

fn error(msg: impl Display){
    eprintln!("{}", formatted("error", "\x1b[31m", msg));
    std::process::exit(1)
}

fn ok(msg: impl Display){
    println!("{}", formatted("success", "\x1b[32m", msg))
}

fn main(){
    let mut release = false;
    let mut clean = false;
    let mut rm_src = false;
    
    let intro = "this is a script for compiling and bundling wasm for a static website";

    for arg in std::env::args().next(){
        match arg.as_str() {
            "-h" | "--help" => {                
            println!(
                "{intro} - showing help\noptions:\n
                \t-r\t--release\tuse rust compiler in release mode instead of debug\n
                \t-c\t--clean\tremove files from last build\n
                \t\t--rm-src\tremoves all development files!!!"
            );
            std::process::exit(0);
            },
            "-t" | "--release" => {release = true},
            "-c" | "--clean" => {clean = true},
            "--rm-src" => {
                println!("are you sure you want to remove all development files? they cannot be recovered\n\ty=YES, n=NO");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf);
                match buf.to_lowercase().as_str() {
                    "y"=>{rm_src = true;},
                    "n"=>{std::process::exit(0)},
                    _=>{error(format!("{buf} is not valid, try y for YES, n for NO"));}
                }
            },
            _ => {error(format!("{arg} is not valid"))}
        }
    }

    println!("{}", intro);

    use std::env::consts::OS;
    match OS {
        "windows" | "linux" =>{},
        _ => {warn(format!("script not tested for {OS}, 
        supported platforms: [windows, linux]\n
        \tif it fails, try building manually with cargo, wasm-bindgen cli and moving index.html, see README.md"))}
    }


    cargo::ops::run(".", )

}