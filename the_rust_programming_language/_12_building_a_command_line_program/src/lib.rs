// - "IGNORE_CASE=1 cargo run -- to poem.txt"
// - Powershell
//   - "$Env:IGNORE_CASE=1; cargo run -- to poem.txt"

pub fn run() {
    let args_config: ArgsConfig = ArgsConfig::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    let contents: String = std::fs::read_to_string(&args_config.file_path).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    });

    for line in search(&args_config.query, &contents, args_config.ignore_case) {
        println!("{line}");
    }
}

struct ArgsConfig {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl ArgsConfig {
    fn build() -> Result<Self, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut ignore_case = std::env::var("IGNORE_CASE").is_ok();

        // Let's just assume this for the exercise
        if args.len() >= 4 && args[3] == "--ignore-case" {
            ignore_case = true;
        }

        Ok(Self { query: args[1].clone(), file_path: args[2].clone(), ignore_case })
    }
}

fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let query = if ignore_case { &query.to_lowercase() } else { query };

    contents
        .lines()
        .filter(|line| {
            let line = if ignore_case { &line.to_lowercase() } else { *line };
            line.contains(query)
        })
        .map(|filtered_line| filtered_line.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

    #[test]
    fn one_result() {
        let query: &str = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS, false));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, CONTENTS, true));
    }
}
