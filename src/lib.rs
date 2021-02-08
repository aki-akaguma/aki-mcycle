//!
//! the substitude text program.
//!
//! ```text
//! Usage:
//!   aki-mcycle [options]
//!
//! mark up text with cycling color.
//!
//! Options:
//!   -e, --exp <exp>      regex (default: ' ([0-9A-Z]{3,}):')
//!
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//!
//! Env:
//!   RUST_CYCLE_COLOR_RED_ST      red start sequence
//!   RUST_CYCLE_COLOR_GREEN_ST    green start sequence
//!   RUST_CYCLE_COLOR_BLUE_ST     blue start sequence
//!   RUST_CYCLE_COLOR_CYAN_ST     cyan start sequence
//!   RUST_CYCLE_COLOR_MAGENDA_ST  magenda start sequence
//!   RUST_CYCLE_COLOR_YELLOW_ST   yellow start sequence
//!   RUST_CYCLE_COLOR_ED          color end sequence
//! ```
//!

#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute mcycle
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "gsub"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::medium::stdioe::{StreamInStdin,StreamOutStdout,StreamErrStderr};
/// use runnel::StreamIoe;
///
/// let r = libaki_mcycle::execute(&StreamIoe{
///     sin: Box::new(StreamInStdin::default()),
///     sout: Box::new(StreamOutStdout::default()),
///     serr: Box::new(StreamErrStderr::default()),
/// }, "cycle-color", &["-e", "Message:"]);
/// ```
///
pub fn execute(sioe: &StreamIoe, program: &str, args: &[&str]) -> anyhow::Result<()> {
    //
    let conf = match conf::parse_cmdopts(program, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.sout.lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
