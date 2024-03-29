pub mod cli {
    use std::{path::PathBuf};
    use rip_grep::{OutputFormat};

    const HELP_OPT: &str = "--help";
    const FORMAT_OPT: &str = "--format";
    const VERBOSE_OPT: &str = "--verbose";

    #[derive(Debug)]
    pub struct Cli {
        pub pattern: String,
        pub path: PathBuf,
        pub path_str: String,
        pub verbose: bool,
        pub format: OutputFormat,
        pub is_help: bool,
        pub options: Vec<String>,
    }

    impl Cli {
        pub fn parse() -> Cli {
            let mut args = std::env::args();
            let options = read_options();
            let copied_options = options.to_vec();

            let mut has_help_opt = false;
            let mut verbose = false;
            let mut format: OutputFormat = OutputFormat::PlainText;

            for opt in options {
                // I don't understand enums in Rust :)
                if opt.starts_with(HELP_OPT) {
                    has_help_opt = true;
                } else if opt.starts_with(VERBOSE_OPT) {
                    let opt_value = get_option_value(&opt);
                    verbose = match opt_value {
                       "false" => false,
                       _ => true,
                    }
                } else if opt.starts_with(FORMAT_OPT) {
                    let opt_value = get_option_value(&opt);

                    format = match opt_value {
                       "JSON" | "json" => OutputFormat::JSON,
                       _ => OutputFormat::PlainText,
                    }
                }
            }

            let pattern = args.nth(args.len() - 2).expect("Missing: `pattern`. Example: `rig_grep Hello .`");
            let path = args.last().expect("Missing: `path`. Example: `rig_grep Hello .`");
            let path_buff = parse_path(&path);

            Cli {
                path_str: path,
                pattern: pattern,
                path: path_buff,
                is_help: has_help_opt,
                verbose: verbose,
                format: format,
                options: copied_options,
            }
        }

        pub fn show_help() {
            println!("Welcome to simple ripgrep implementation made by VladM during learning Rust");
            println!("Options: ");
            println!("--help: Call help, I guess you know it already :)");
            println!("--verbose: values: 'true', 'false'. Show folders and files that are scanned");
            println!("--format: values: 'JSON', 'PlainText'. Return format");
            println!("Example of usage: `rig_grep Hello .");
        }
    }

    fn read_options() -> Vec<String> {
        let mut options = Vec::new();
        for arg in std::env::args().skip(1) {
           options.push(arg);
        }

        return options;
    }

    fn parse_path(path: &String) -> PathBuf {
        PathBuf::from(path)
    }

    fn get_option_value(opt: &str) -> &str {
        let opt_split: Vec<&str> = opt.split("=").collect();
        opt_split.get(1)
            .expect("Failed to get option value")
    }
}
