use clap::{Arg, Command, Parser};
use std::process::Command as exec;
use glob::glob;
use ansi_term::Style;
use ansi_term::Colour::Red;


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
                        .help("Install hardlinks for all subcommands in path")
                        .exclusive(true)
                        .takes_value(true)
                        .default_missing_value("modele.ScenarioDonnee")
                        .use_value_delimiter(false),
                )
        )
        .get_matches();

    let subcommand = matches.subcommand();
    let verbose: bool = matches.is_present("verbose");
    match subcommand {
        Some(("run", _)) => {
            let args = matches.subcommand_matches("run").unwrap();
            let mut file = "modele.ScenarioDonnee";
            if args.is_present("file") {
                file = args.values_of("file").unwrap().next().unwrap();
                println!("{}", file);

            }
            run(verbose, file);
        },
        Some(("build", _)) => {
            build(verbose);
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

fn run(v: bool, file: &str) {
    build(v);

    println!("verbose: {}", v);

    let str: String = format!("[+] Running {}", &file);
    println!("{}", Style::new().bold().paint(str));

    let out = exec::new("java")
        .arg("-classpath")
        .arg("class/")
        .arg(file)
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));
}

fn build(verbose: bool) {
    let enums: Vec<&str> = vec!["ContenuNid", "EspeceBatracien", "EspeceChouette", "EspeceHippocampe", "EspeceObservee", "IndiceLoutre", "Peche", "Sexe", "TypeObservation"];
    let venums: Vec<String> = enums.iter().map(|x| format!("src/modele/donnee/{}.java", x)).collect();

    let files: Vec<String> = read_dir("./src/**/*.java");

    let str: String = format!("[+] Building {} {}", &venums.join(" "), &files.join(" "));
    println!("{}", Style::new().bold().paint(str));

    let out = exec::new("javac")
        .arg("-classpath")
        .arg("class/")
        .arg("-d")
        .arg("class/")
        .arg("-encoding")
        .arg("UTF-8")
        .args(venums)
        .args(files)
        .arg(if verbose { "-verbose" } else { "-Xdoclint" })
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));
}

fn doc() {
    let files: Vec<String> = read_dir("./src/**/*.java");

    println!("[+] Javadoc {}", &files.join(" "));

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
