mod command;
mod drop_zeroed;
mod error;

use command::run_command;
use error::Result;

fn try_main() -> Result<i32> {
    let status = run_command(argv::iter().skip(1))?;

    if !status.is_ok() {
        println!("[kindly] {}", status);
    }

    Ok(status.code_or_signal())
}

fn main() {
    match try_main() {
        Ok(code_or_signal) => std::process::exit(code_or_signal),
        Err(err) => {
            println!("[kindly] {}", err);
            std::process::exit(127);
        }
    }
}
