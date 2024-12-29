use std::io::Repeat;

use error_stack::{Report, ResultExt};
use track::{error::{AppError, Suggestion}, init};

fn main() -> Result<(), Report<AppError>> {
    init::error_reporting();

    // return Err(Report::from(AppError))
    // .attach(Suggestion("do something else"))
    // .attach_printable("a  printable value")
     Ok(())
}
