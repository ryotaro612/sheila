use env_logger::Builder;
use log::LevelFilter;

/**
  Sets the logging level.
  If verbose is true, the log module prints messages with level Debug or higher.
  Otherwise, the threshold is Info.
*/
pub(crate) fn init_log(verbose: bool) {
    let mut builder = Builder::new();
    if verbose {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Info);
    }
    builder.init();
    if verbose {
        log::debug!("verbose mode is on");
    }
}
