use std::fs;
use std::path::Path;
use std::process::{Command as exec, exit};

use ansi_term::{Colour::Red, Style};
use clap::{Arg, Command, Parser};
use glob::glob;

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
                        .short("f".parse().unwrap())
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
        .subcommand(
            Command::new("test")
                .about("Run JUnit tests")
                .subcommand_help_heading("TEST")
        )
        .get_matches();

    let lib: String = get_lib_path();
    let subcommand = matches.subcommand();
    let verbose: bool = matches.is_present("verbose");
    let mut file: String = "".to_string();
    match subcommand {
        Some(("run", _)) => {
            let args = matches.subcommand_matches("run").unwrap();
            if args.is_present("file") {
                file = args.values_of("file").unwrap().next().unwrap().parse().unwrap();
            }
            run(verbose, &file, &lib);
        }
        Some(("build", _)) => {
            build(verbose, &lib, &file);
        }
        Some(("doc", _)) => {
            doc();
        }
        Some(("test", _)) => {
            test(&lib);
        }
        Some(("", _)) => {
            eprintln!("{}", Red.paint("No command specified, try --help"));
        }
        _ => {
            println!();
        }
    }
}

fn build(verbose: bool, lib: &String, file: &String) {
    // Get all the .java files and compile them
    let enums: Vec<&str> = vec!["ContenuNid", "EspeceBatracien", "EspeceChouette", "EspeceHippocampe", "EspeceObservee", "IndiceLoutre", "Peche", "Sexe", "TypeObservation"];
    let mut venums: Vec<String> = enums.iter().map(|x| format!("src/modele/donnee/{}.java", x)).collect();

    let mut files: Vec<String>;
    if file == "" {
        files = vec![];
        files.append(&mut venums);
        files.append(&mut read_dir("./src/**/*.java"));
    } else {
        files = vec![format_filename(file)];
        if file.starts_with("vue.") {
            let controller: String = format!("src/controleur/{}Controller.java", file.split(".").last().unwrap());
            if Path::new(&controller).exists() {
                files.push(controller);
            }
        }
    }

    println!("{} {}", Style::new().bold().paint("[+] Building: "), &files.join(", "));

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
        .args(files)
        .arg(if verbose { "-verbose" } else { "-Xdoclint:none" })
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));

    if !out.status.success() {
        eprintln!("{}", Style::new().bold().paint("[!] Build failed"));
        exit(1);
    }


    // Get all the complementary files (.fxml, .css) and copy them to the class dir
    copy_files();
}

fn run(v: bool, file: &String, lib: &String) {
    build(v, lib, &file);

    let run_file = if file == "" {
        "controleur.Main".to_string()
    } else {
        format_filename(file)
    };

    println!("{} {}", Style::new().bold().paint("[+] Running: "), &run_file);

    let out = exec::new("java")
        .arg("-classpath")
        .arg(get_classpath())
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics,javafx.fxml,javafx.media,javafx.swing,javafx.web")
        .arg(&run_file)
        .output()
        .expect(format!("[!] Failed to run {}", &run_file).as_str());

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


fn test(lib: &String) {
    let files = read_dir("./src/test/*.java");

    println!("{} {}", Style::new().bold().paint("[+] Building: "), &files.join(", "));

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
        .args(&files)
        .output()
        .expect("[!] Failed to compile");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", Red.paint(String::from_utf8_lossy(&out.stderr)));

    if !out.status.success() {
        eprintln!("{}", Style::new().bold().paint("[!] Build failed"));
        exit(1);
    }

    let mut run_files: Vec<String> = Vec::new();
    for file in files {
        run_files.push(format_classname(&file));
    }

    println!("{} {}", Style::new().bold().paint("[+] Running: "), &run_files.join(", "));

    let out = exec::new("java")
        .arg("-classpath")
        .arg(get_classpath())
        .arg("--module-path")
        .arg(lib)
        .arg("--add-modules")
        .arg("javafx.base,javafx.controls,javafx.graphics,javafx.fxml,javafx.media,javafx.swing,javafx.web")
        .arg("org.junit.runner.JUnitCore")
        .args(&run_files)
        .output()
        .expect("[!] Failed to run");

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
        assert!(Path::new("/usr/lib/jvm/openjfx").exists(), "OpenJFX not found. Please install openjfx-devel.");
        path = "/usr/lib/jvm/openjfx".to_string();
    }

    path
}

fn get_classpath() -> String {
    let mut path: String = "class;lib\\mysql-connector-java-8.0.29.jar;lib\\annotations-20.1.0.jar;lib\\junit-4.13.2.jar;lib\\hamcrest-core-1.3.jar".to_string();

    if !cfg!(target_os = "windows") {
        path = "class:lib/mysql-connector-java-8.0.29.jar:lib/annotations-20.1.0.jar:lib/junit-4.13.2.jar:lib/hamcrest-core-1.3.jar".to_string();
    }

    path
}

fn format_filename(file: &String) -> String {
    let mut file: String = file.replace(".", "/");
    if file != "" {
        file = format!("src/{}.java", file)
    }
    file
}

fn format_classname(file: &String) -> String {
    let mut file: String = file.replace("src/", "").replace("src\\", "").replace(".java", "").replace("/", ".").replace("\\", ".");
    if file != "" {
        file = format!("{}", file)
    }
    file
}

fn copy_files() {
    let mut comp_files: Vec<String> = read_dir("./src/**/*.fxml");
    comp_files.append(&mut read_dir("./src/**/*.css"));

    println!("{} {} {}", Style::new().bold().paint("[+] Copying: "), &comp_files.join(", "), "to class");

    for file in comp_files {
        let split_char = if !cfg!(target_os = "windows") { "/" } else { "\\" };
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
