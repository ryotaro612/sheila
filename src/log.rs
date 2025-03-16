use env_logger::Builder;
use log::LevelFilter;
/** 
 *  Initialize the log system
*/
pub(crate) fn init_log(verbose: bool) {
    let mut builder = Builder::new();
    if verbose {
        builder.filter_level(LevelFilter::Debug);
    } else {
      builder.filter_level(LevelFilter::Info);
    }
    builder.init();
}