use std::process::ExitCode;

fn main() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(1);
        }
    };

    match new_alphabet_cli::run_from(std::env::args(), &cwd) {
        Ok(result) => {
            if !result.stdout.is_empty() {
                println!("{}", result.stdout);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
