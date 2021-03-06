//
use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

use crate::conf::CmdOptConf;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
mark up text with cycling color.
"#;
//const ARGUMENTS_TEXT: &str = r#""#;
const ENV_TEXT: &str = r#"Env:
  AKI_MCYCLE_COLOR_RED_ST       red start sequence
  AKI_MCYCLE_COLOR_GREEN_ST     green start sequence
  AKI_MCYCLE_COLOR_BLUE_ST      blue start sequence
  AKI_MCYCLE_COLOR_CYAN_ST      cyan start sequence
  AKI_MCYCLE_COLOR_MAGENDA_ST   magenda start sequence
  AKI_MCYCLE_COLOR_YELLOW_ST    yellow start sequence
  AKI_MCYCLE_COLOR_ED           color end sequence
"#;
//const EXAMPLES_TEXT: &str = r#""#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ENV_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(program: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf::create(program);
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(
            &conf.opt_program,
        )));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.opt_program,
        )));
        return Err(errs);
    }
    //
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        if conf.opt_exp.is_empty() {
            errs.push(OptParseError::missing_option("e"));
        }
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                errs.push(OptParseError::unexpected_argument(&free[0]));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
