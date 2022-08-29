use cfg_log::*;

#[cfg(feature = "log")]
use log::Level;

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Cannot initialize the logger");

    if log_enabled!(Level::Debug) {
        println!("Debug enabled");
    } else {
        println!("Debug disabled");
    }

    log!(Level::Info, "anwer = {}", 42);
    trace!("the answer is {}", 42);
    debug!("the answer is {}", 42);
    info!("the answer is {}", 42);
    warn!("the answer is {}", 42);
    error!("the answer is {}", 42);

    log!(target: "tgt", Level::Info, "anwer = {}", 42);
    trace!(target: "tgt", "the answer is {}", 42);
    debug!(target: "tgt", "the answer is {}", 42);
    info!(target: "tgt", "the answer is {}", 42);
    warn!(target: "tgt", "the answer is {}", 42);
    error!(target: "tgt", "the answer is {}", 42);
}
