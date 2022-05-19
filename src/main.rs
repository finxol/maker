use clap::Parser;
use std::process::Command;
use glob::glob;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    action: Option<String>,
}


fn main() {
    let args: Cli = Cli::parse();

    if args.action.is_some() {
        let arg: String = args.action.unwrap();
        if arg == "run" {
            run();

        } else if arg == "build" {
            build();

        } else if arg == "doc" {
            doc();

        } else {
            println!("Unrecognized action: {}, try --help", arg);
        }
    } else {
        println!("Unrecognized action, try --help");
    }
}

fn run() {
    build();

    let file: &str = "modele.ScenarioDonnee";

    println!("[+] Running {}", &file);

    let out = Command::new("java")
        .arg("-classpath")
        .arg("class/")
        .arg(file)
        .output()
        .expect("[!] Failes to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
}

fn build() {
    let enums: Vec<&str> = vec!["ContenuNid", "EspeceBatracien", "EspeceChouette", "EspeceHippocampe", "EspeceObservee", "IndiceLoutre", "Peche", "Sexe", "TypeObservation"];
    let venums: Vec<String> = enums.iter().map(|x| format!("./src/modele/donnee/{}.java", x)).collect();

    let files: Vec<String> = read_dir("./src/**/*.java");


    println!("[+] Building {} {}", &venums.join(" "), &files.join(" "));

    let out = Command::new("javac")
        .arg("-classpath")
        .arg("class/")
        .arg("-d")
        .arg("class/")
        .args(venums)
        .args(files)
        .output()
        .expect("[!] Failes to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
}

fn doc() {
    let files: Vec<String> = read_dir("./src/**/*.java");

    println!("[+] Javadoc {}", &files.join(" "));

    let out = Command::new("javadoc")
        .arg("-d")
        .arg("doc/")
        .arg("-charset")
        .arg("UTF-8")
        .arg("-classpath")
        .arg("class/")
        .arg("-author")
        .args(files)
        .output()
        .expect("[!] Failes to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
}


fn read_dir(dir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    for entry in glob(dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path.to_str().unwrap().to_string()),
            Err(e) => println!("{:#?}", e),
        }
    }

    files
}
