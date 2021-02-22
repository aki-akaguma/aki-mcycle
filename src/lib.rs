//!
//! the mark up text with cycling color program.
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
//!
//! # Examples
//!
//! ### Command line example 1
//!
//! Extract "`arm`" from the rustup target list and make "`linux-[^ ]+`" **color**.
//!
//! - 1st match: makes '`linux-musl`' **red**
//! - 2nd match: makes '`linux-musleabi`' **green**
//! - 3rd match: makes '`linux-musleabihf`' **blue**
//! - 4th match: makes '`linux-muslabi64`' **cyan**
//!
//! ```text
//! rustup target list | aki-mline -e arm | aki-mcycle -e "linux-[^ ]+"
//! ```
//!
//! result output :
//!
//! ![out rustup image]
//!
//! [out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcycle/main/img/out-rustup-1.png
//!
//! - [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.
//!
//! ### Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
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
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_mcycle::execute(&RunnelIoeBuilder::new().build(),
///     "mcycle", &["-e", "Message: *[^ ]+"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
