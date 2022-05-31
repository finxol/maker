use std::path::Path;
use clap::{Arg, Command, Parser};
use std::process::Command as exec;
use glob::glob;
use ansi_term::{Style, Colour::Red};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    action: Option<String>,
}


fn main() {
    let matches = Command::new("Maker")
        .author("finxol")
        .about("Easily run a java project")
        .arg(
            Arg::new("verbose")
                .help("Run in verbose mode")
                .takes_value(false)
                .short("v".parse().unwrap())
                .long("verbose")
                .global(true),
        )
        .subcommand(
            Command::new("run")
                .about("Run the application or a specific class")
                .subcommand_value_name("file")
                .subcommand_help_heading("RUN")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .help("Specify which file to run")
                        .exclusive(true)
                        .takes_value(true)
                        .default_missing_value("modele.ScenarioDonnee")
                        .use_value_delimiter(false),
                )
        )
        .subcommand(
            Command::new("build")
                .about("Build the application")
                .subcommand_help_heading("BUILD")
        )
        .subcommand(
            Command::new("doc")
                .about("Generate documentation")
                .subcommand_help_heading("DOC")
        )
        .get_matches();

    let lib: String = get_lib_path();
    let subcommand = matches.subcommand();
    let verbose: bool = matches.is_present("verbose");
    match subcommand {
        Some(("run", _)) => {
            let args = matches.subcommand_matches("run").unwrap();
            let mut file = "modele.ScenarioDonnee";
            if args.is_present("file") {
                file = args.values_of("file").unwrap().next().unwrap();
            }
            run(verbose, file, &lib);
        },
        Some(("build", _)) => {
            build(verbose, &lib);
        },
        Some(("doc", _)) => {
            doc();
        },
        Some(("", _)) => {
            eprintln!("{}", Red.paint("No command specified, try --help"));
        },
        _ => {
            println!();
        }
    }
}

fn build(verbose: bool, lib: &String) {
    let enums: Vec<&str> = vec!["ContenuNid", "EspeceBatracien", "EspeceChouette", "EspeceHippocampe", "EspeceObservee", "IndiceLoutre", "Peche", "Sexe", "TypeObservation"];
    let venums: Vec<String> = enums.iter().map(|x| format!("src/modele/donnee/{}.java", x)).collect();

    let files: Vec<String> = read_dir("./src/**/*.java");

    println!("{} {} {}", Style::new().bold().paint("[+] Building: "), &venums.join(", "), &files.join(", "));

    let out = exec::new("javac")
        .arg("-classpath")
        .arg("class/")
        .arg("-d")
        .arg("class/")
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics")
        .arg("-encoding")
        .arg("UTF-8")
        .args(venums)
        .args(files)
        .arg(if verbose { "-verbose" } else { "-Xdoclint:none" })
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));
}

fn run(v: bool, file: &str, lib: &String) {
    build(v, lib);

    println!("verbose: {}", v);

    println!("{} {}", Style::new().bold().paint("[+] Running: "), &file);

    let out = exec::new("java")
        .arg("-classpath")
        .arg("class/")
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics")
        .arg(file)
        .output()
        .expect(format!("[!] Failed to run {}", file).as_str());

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));
}

fn doc() {
    let files: Vec<String> = read_dir("./src/**/*.java");

    println!("{} {}", Style::new().bold().paint("[+] Javadoc: "), &files.join(" "));

    let out = exec::new("javadoc")
        .arg("-d")
        .arg("doc/")
        .arg("-classpath")
        .arg("class/")
        .arg("-author")
        .args(files)
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));
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

fn get_lib_path() -> String {
    let mut path: String = "lib/lib".to_string();

    if !cfg!(target_os = "windows") {
        assert!(Path::new("/usr/lib/jvm/openjfx").exists(), "OpenJFX not found");
        path = "/usr/lib/jvm/openjfx".to_string();
    }

    path
}
