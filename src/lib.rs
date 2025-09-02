/*!
the mark up text with cycling color program.

# Features

- mark up text with cycling color.
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

# Command help

```text
aki-mcycle --help
```

```text
Usage:
  aki-mcycle [options]

mark up text with the cyclic color.

Options:
  -e, --exp <exp>   write it in the cyclic color (default: ' ([0-9A-Z]{3,}):')

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <exp>     regular expression, color the entire match with the cyclic color.

Environments:
  AKI_MCYCLE_COLOR_SEQ_RED_ST       red start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_GREEN_ST     green start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_ED           color end sequence specified by ansi
```

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-mcycle
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Command line example 1

Extract "`arm`" from the rustup target list and make "`linux-[^ ]+`" **color**.

- 1st match: makes '`linux-musl`' **red**
- 2nd match: makes '`linux-musleabi`' **green**
- 3rd match: makes '`linux-musleabihf`' **blue**
- 4th match: makes '`linux-muslabi64`' **cyan**

```text
rustup target list | aki-mline -e arm | aki-mcycle -e "linux-[^ ]+"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcycle/main/img/out-rustup-1.png

- [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

*/
#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute mcycle
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "mcycle"
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
///     "mcycle", ["-e", "Message: *[^ ]+"]);
/// ```
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env<I, S>(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: I,
    env: &conf::EnvConf,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    //
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf, env),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
