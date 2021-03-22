//
use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
mark up text with the cyclic color.
"#;
const PARAMS_TEXT: &str = r#"Option Parameters:
  <exp>     regular expression, color the entire match with the cyclic color.
"#;
//const ARGUMENTS_TEXT: &str = r#""#;
const ENV_TEXT: &str = r#"Environments:
  AKI_MCYCLE_COLOR_SEQ_RED_ST       red start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_GREEN_ST     green start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi
  AKI_MCYCLE_COLOR_SEQ_ED           color end sequence specified by ansi
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
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, PARAMS_TEXT, ENV_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf {
        prog_name: a_prog_name.to_string(),
        opt_exp: " ([0-9A-Z]{3,}):".to_string(), // default value
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(&conf.prog_name)));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
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
