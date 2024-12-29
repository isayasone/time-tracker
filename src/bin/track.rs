use std::io::Repeat;

use error_stack::{Report, ResultExt};
use tracing::{info, instrument, warn};
use track::{error::{AppError, Suggestion}, init};


// #[instrument]
// fn a(arg:i8)
// {
//     info!("an event");
// }


fn main() -> Result<(), Report<AppError>> {
    init::error_reporting();
    init::tracing();
       
    //   a(5); 
    // warn!(" a waring");
    // info!("an info");
    // trace info $ RUST_LOG=warn cargo run 
    // return Err(Report::from(AppError))
    // .attach(Suggestion("do something else"))
    // .attach_printable("a  printable value")
     Ok(())
}






