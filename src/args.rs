use std::env;

/// Returns (flags, args, preserved)
pub fn parse(use_preserve: bool) -> (Vec<String>, Vec<String>, Vec<String>) {
    let args = env::args().collect::<Vec<_>>();

    let mut flags = Vec::new();
    let mut ret_args = Vec::new();
    let mut preserve = Vec::new();
    let mut is_preserve = false;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        if arg == "--" && use_preserve {
            is_preserve = true;
        } else if arg.starts_with("-") {
            if arg == "-" {
                ret_args.push(arg.to_owned());
            } else {
                flags.push(resolve(arg.to_owned()));
            }
        } else if is_preserve {
            preserve.push(arg.to_owned());
        } else {
            ret_args.push(arg.to_owned());
        }

        i += 1;
    }

    (flags, ret_args, preserve)
}

fn resolve(arg: String) -> String {
    match arg.as_str() {
        "-V" => "verbose".to_string(),
        "-vv" => "very-verbose".to_string(),
        "-h" => "help".to_string(),
        "-v" => "version".to_string(),
        "-o" => "out".to_string(),
        _ => arg.replace('-', ""),
    }
}
