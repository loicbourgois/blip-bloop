use std::env;
use std::process::Command;

fn help() {
    println!("commands:");
    println!("    bb help       Display this help");
    println!("    bb readme     Open README.md");
    println!("    bb web        Open blip-bloop-web/README.md and blip-bloop-web/index.html");
    println!("    bb web build  ");
}

fn open(path: &str) {
    println!("Opening {}", path);
    let _output = Command::new("sh")
        .arg("-c")
        .arg(format!("open {}", path))
        .output()
        .unwrap();
}

fn web_build() {
    println!("web build");
}

fn readme() {
    open("$HOME/github.com/loicbourgois/blip-bloop/README.md");
}

fn web() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("rustup run nightly wasm-pack build $HOME/github.com/loicbourgois/blip-bloop/blip-bloop-web")
        .output()
        .unwrap();
    println!("status_code:");
    println!("{}", output.status.code().unwrap());
    println!("stdout({}):", output.stdout.len());
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("stderr({}):", output.stderr.len());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    match output.status.code().unwrap() {
        0 => println!("ok"),
        _ => println!("not ok"),
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("npm install $HOME/github.com/loicbourgois/blip-bloop/blip-bloop-web/www")
        .output()
        .unwrap();
    println!("status_code:");
    println!("{}", output.status.code().unwrap());
    println!("stdout({}):", output.stdout.len());
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("stderr({}):", output.stderr.len());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    match output.status.code().unwrap() {
        0 => println!("ok"),
        _ => println!("not ok"),
    }

    println!("starting dev server at http://localhost:8080/");
    let output = Command::new("sh")
        .arg("-c")
        .arg("npm --prefix $HOME/github.com/loicbourgois/blip-bloop/blip-bloop-web/www run start")
        .output()
        .unwrap();
    println!("status_code:");
    println!("{}", output.status.code().unwrap());
    println!("stdout({}):", output.stdout.len());
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("stderr({}):", output.stderr.len());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    match output.status.code().unwrap() {
        0 => println!("ok"),
        _ => println!("not ok"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_str() {
                "readme" => readme(),
                "web" => web(),
                _ => help()
            }
        }
        3 => {
            match (args[1].as_str(), args[2].as_str()) {
                ("web", "build") => {
                    web_build()
                }
                _ => help()
            }
        }
        _ => help()
    }
}
