use thiserror::Error as DError;

#[derive(DError, Debug)]
enum Error {
    #[error("Oh no!")]
    Test,
}

const ALLOW_PARAM_PASS: bool = true;

macro_rules! ok_or_none {
    ($expr:expr) => {{
        if ALLOW_PARAM_PASS {
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
    let x: Option<u32> = ok_or_none!(this_passes())?;

    let y = ok_or_none!(this_errors())?;

    println!("x: {x:?}, y: {y:?}");

    Ok(())
}
