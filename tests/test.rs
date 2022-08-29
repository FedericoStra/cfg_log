use cfg_log::*;

#[cfg(feature = "log")]
use log::Level;

#[test]
fn log() {
    log!(Level::Info, "Hello");
    log!(Level::Info, "The answer is {}", 42);
    log!(target: "my_target", Level::Info, "Bye!");
}

#[test]
fn trace() {
    trace!("Hello");
    trace!("The answer is {}", 42);
    trace!(target: "my_target", "Bye!");
}

#[test]
fn debug() {
    debug!("Hello");
    debug!("The answer is {}", 42);
    debug!(target: "my_target", "Bye!");
}

#[test]
fn info() {
    info!("Hello");
    info!("The answer is {}", 42);
    info!(target: "my_target", "Bye!");
}

#[test]
fn warn() {
    warn!("Hello");
    warn!("The answer is {}", 42);
    warn!(target: "my_target", "Bye!");
}

#[test]
fn error() {
    error!("Hello");
    error!("The answer is {}", 42);
    error!(target: "my_target", "Bye!");
}
