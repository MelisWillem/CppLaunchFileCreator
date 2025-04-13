use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct SetupCommand{
    description: String,
    text: String,
    #[serde(rename="ignoreFailures")]
    ignore_failures: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    r#type: String,
    request: String,
    args: Vec<String>,
    program: String,
    cwd: String,
    #[serde(rename="setupCommands")]
    setup_commands: Vec<SetupCommand>
}

impl Config {
    fn new(binary_path_string: &String, arguments: Vec<String>) -> Result<Config, String> {
        let binary_path = match std::fs::canonicalize(PathBuf::from(binary_path_string)) {
            Ok(path) => path,
            Err(_) => return Err(format!("The binary path({binary_path_string}) is invalid. Either the file doesn't exist or one of the directories in the path doesn't exist.")),
        };

        // The cwd is set to the current directory, to simulate the behavior of 'gdb --args ./a.out arg1 arg2 arg3'
        let current_dir = match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => {
                return Err(format!("Can't get the current directory.").into());
            }
        };
        let current_dir = match current_dir.to_str() {
            Some(s) => s.to_string(),
            None => return Err(format!("The current directory is invalid. Can't create a string from the path.")),
        };

        let binary_path = match binary_path.to_str() {
            Some(s) => s,
            None => return Err(format!("The binary path({binary_path_string}) is invalid. Can't create a string from the path.")),
        };

        let setup_commands = vec![SetupCommand{
            description: "Enable pretty-printing for gdb".into(),
            text: "-enable-pretty-printing".into(),
            ignore_failures: true
        }];

        Ok(Config {
            name: "c++ launch".to_string(),
            r#type: "cppdbg".to_string(),
            request: "launch".to_string(),
            args: arguments,
            program: binary_path.to_string(),
            cwd: current_dir,
            setup_commands
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

static EXAMPLE_CMD: &str = "example: 'clfc a.out arg1 arg2 arg3'";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    // layout arguments
    // 0=> rust binary of this code
    // 1=> cpp binary
    // 1< => cpp arguments (optional)

    if args.len() > 1 && (&args[1] == "--help" || &args[1] == "-h" || &args[1] == "-help") {
        println!("cpp launch file creator:\n Outputs launch.json file (to put in .vscode, and is consumed by c/c++ extension from microsoft) to run a command line throught gdb. The equivalent of 'gdb --args a.out arg1 arg2 arg3' . \n {EXAMPLE_CMD}");
        return Ok(());
    }

    // skip the first argument (rust binary name) and get the second argument (cpp binary name)
    let binary_path = match args.iter().skip(1).next() {
        Some(path) => path.clone(),
        None => {
            println!();
            return Err(format!("Please provide the binary name. {EXAMPLE_CMD}").into());
        }
    };

    let debug_arguments: Vec<String> = args
        .into_iter()
        .skip(2) // skip both the rust and c++ binary name
        .collect();

    let config = match Config::new(&binary_path, debug_arguments) {
        Ok(config) => config,
        Err(error) => {
            return Err(format!("{error}").into());
        }
    };
    let launch = Launch::new(config);

    let launch_json = serde_json::to_string_pretty(&launch)
        .expect("Internal error: can't create launch json file.");

    println!("{launch_json}");

    return Ok(());
}
