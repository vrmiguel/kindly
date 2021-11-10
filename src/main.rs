use command::run_command;

mod command;
mod drop_zeroed;
mod error;

fn main() {
    run_command(argv::iter().skip(1));
}
