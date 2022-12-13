use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    r#type: String,
    request: String,
    args: Vec<String>,
    program: String,
    cwd: String,
}

impl Config {
    fn new(binary_path_string: &String, arguments: Vec<String>) -> Result<Config, String> {
        let binary_path = std::fs::canonicalize(PathBuf::from(binary_path_string)).unwrap();


        if !binary_path.exists()
        {
            return Err(match binary_path.to_str() {
                Some(s) => format!("The binary path {s}"),
                None => format!("The binary path({binary_path_string}) is invalid.")
            });
        }

        Ok(Config {
            name: "c++ launch".to_string(),
            r#type: "cppdbg".to_string(),
            request: "launch".to_string(),
            args: arguments,
            program: binary_path.to_str().unwrap().to_string(),
            cwd: "${workspaceFolder}".to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Launch {
    configurations: Vec<Config>,
}

impl Launch {
    fn new(config: Config) -> Launch {
        let configs = vec![config];
        Launch {
            configurations: configs,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // layout arguments
    // 0=> rust binary of this code
    // 1=> cpp binary
    // 1< => cpp arguments (optional)

    if args.len() < 3 {
        println!("Empty run, can't create launch .json");
        println!("You should at least provide one argument to specify the binary you want to run. (example: 'clfc a.out')");
    }

    let binary_path = match args.iter().skip(1).next() {
        Some(path) => path.clone(),
        None => panic!("Please provide the binary name"),
    };

    let debug_arguments: Vec<String> = args
        .into_iter()
        .skip(2) // skip both the rust and c++ binary name
        .map(|x| format!("\"{x}\""))
        .collect();

    let config = Config::new(&binary_path, debug_arguments).unwrap_or_else(|error| panic!("{error}"));
    let launch = Launch::new(config);

    let launch_json =
        serde_json::to_string_pretty(&launch).expect("Internal error: can't create launch json file.");

    println!("{launch_json}");
}
