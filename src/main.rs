use std::process::ExitCode;

use std::io;

use rs_uuids2tinybloom::stdin2uuids2bloom2stdout_raw;

fn sub() -> Result<(), io::Error> {
    stdin2uuids2bloom2stdout_raw()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
