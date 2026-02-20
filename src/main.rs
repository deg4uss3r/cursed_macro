use thiserror::Error as DError;
use clap::Parser;

use std::sync::Mutex;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    allow_pass: Option<bool>,
}

#[derive(DError, Debug)]
enum Error {
    #[error("Oh no!")]
    Test,
}

static ALLOW_PARAM_PASS: Mutex<bool> = Mutex::new(false);

macro_rules! ok_or_none {
    ($expr:expr) => {{
        if *ALLOW_PARAM_PASS.lock().unwrap() {
            match $expr {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("ERR: header logger here: {e}");
                    Ok(None)
                }
            }
        } else {
            $expr
        }
    }};
}

fn this_errors() -> Result<Option<u32>, Error> {
    Err(Error::Test)
}

fn this_passes() -> Result<Option<u32>, Error> {
    println!("from passes!");
    Ok(Some(42))
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(allow_pass) = cli.allow_pass {
        let mut this_run = ALLOW_PARAM_PASS.lock().unwrap();
        *this_run = allow_pass;
    }

    let x: Option<u32> = ok_or_none!(this_passes())?;

    let y = ok_or_none!(this_errors())?;

    println!("x: {x:?}, y: {y:?}");

    Ok(())
}
