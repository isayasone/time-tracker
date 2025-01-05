

use error_stack::{Report, ResultExt};
use track::{error::AppError, feature::cli, init};

// #[instrument]
// fn a(arg:i8)
// {
//     info!("an event");
// }

fn main() -> Result<(), Report<AppError>> {
    init::error_reporting();
    init::tracing();

    let _ = cli::run()
        .change_context(AppError)
        .attach_printable("failed to run CLI");

    Ok(())
}
