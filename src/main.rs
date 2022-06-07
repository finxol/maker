use std::path::Path;
use clap::{Arg, Command, Parser};
use std::process::{Command as exec};
use glob::glob;
use ansi_term::{Style, Colour::Red};
use std::fs;


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
                        .default_missing_value("vue.Login")
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
            let mut file = "vue.Login";
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
    // Get all the .java files and compile them
    let enums: Vec<&str> = vec!["ContenuNid", "EspeceBatracien", "EspeceChouette", "EspeceHippocampe", "EspeceObservee", "IndiceLoutre", "Peche", "Sexe", "TypeObservation"];
    let venums: Vec<String> = enums.iter().map(|x| format!("src/modele/donnee/{}.java", x)).collect();

    let files: Vec<String> = read_dir("./src/**/*.java");

    println!("{} {} {}", Style::new().bold().paint("[+] Building: "), &venums.join(", "), &files.join(", "));

    let out = exec::new("javac")
        .arg("-classpath")
        .arg(get_classpath())
        .arg("-d")
        .arg("class/")
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics,javafx.fxml,javafx.media,javafx.swing,javafx.web")
        .arg("-encoding")
        .arg("UTF-8")
        .args(venums)
        .args(files)
        .arg(if verbose { "-verbose" } else { "-Xdoclint:none" })
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));

    if !out.status.success() {
        eprintln!("{}", Style::new().bold().paint("[!] Build failed"));
        panic!();
    }


    // Get all the complementary files (.fxml, .css) and copy them to the class dir
    copy_files();
}

fn run(v: bool, file: &str, lib: &String) {
    build(v, lib);

    println!("verbose: {}", v);

    println!("{} {}", Style::new().bold().paint("[+] Running: "), &file);

    let out = exec::new("java")
        .arg("-classpath")
        .arg(get_classpath())
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics,javafx.fxml,javafx.media,javafx.swing,javafx.web")
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
        .arg(get_classpath())
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

fn get_classpath() -> String {
    let mut path: String = "class;lib\\mysql-connector-java-8.0.29.jar".to_string();

    if !cfg!(target_os = "windows") {
        path = "class:lib/mysql-connector-java-8.0.29.jar".to_string();
    }

    path
}

fn copy_files() {
    let mut comp_files: Vec<String> = read_dir("./src/**/*.fxml");
    comp_files.append(&mut read_dir("./src/**/*.css"));

    println!("{} {} {}", Style::new().bold().paint("[+] Copying: "), &comp_files.join(", "), "to class");

    for file in comp_files {
        let split_char = if !cfg!(target_os = "windows") {
            "/"
        } else {
            "\\"
        };
        let f = file.split(split_char).collect::<Vec<&str>>();
        let f = if !cfg!(target_os = "windows") {
            format!("class/vue/{}", f[f.len() - 1])
        } else {
            format!("class\\vue\\{}", f[f.len() - 1])
        };
        fs::copy(&file, &f)
            .expect(&*format!("Error copying file {} to {}", &file, &f));
    }

    println!();
}
