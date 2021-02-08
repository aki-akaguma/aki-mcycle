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
  RUST_CYCLE_COLOR_RED_ST      red start sequence
  RUST_CYCLE_COLOR_GREEN_ST    green start sequence
  RUST_CYCLE_COLOR_BLUE_ST     blue start sequence
  RUST_CYCLE_COLOR_CYAN_ST     cyan start sequence
  RUST_CYCLE_COLOR_MAGENDA_ST  magenda start sequence
  RUST_CYCLE_COLOR_YELLOW_ST   yellow start sequence
  RUST_CYCLE_COLOR_ED          color end sequence
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
    //[ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ARGUMENTS_TEXT, ENV_TEXT, EXAMPLES_TEXT].join("\n")
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ENV_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}
/*
pub fn parse_my_style<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
) -> (Option<Vec<String>>, Result<(), OptParseErrors>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
    T: HelpVersion,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary);
    let tokens = match lex.tokens_from(&args) {
        Ok(t) => t,
        Err(errs) => {
            return (None, Err(errs));
        }
    };
    //
    let mut errs = OptParseErrors::new();
    //
    for nv in tokens.namevals.iter() {
        match parse_match(conf, &nv) {
            Ok(_) => {}
            Err(err) => {
                errs.push(err);
            }
        }
        if conf.is_help() || conf.is_version() {
            break;
        }
    }
    //
    let mut v: Vec<String> = Vec::new();
    v.extend(tokens.free.iter().map(|&s| s.to_string()));
    //
    return (Some(v), Err(errs));
}
*/

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
