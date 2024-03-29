macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-mcycle [options]

            mark up text with the cyclic color.

            Options:
              -e, --exp <exp>   write it in the cyclic color (default: ' ([0-9A-Z]{3,}):')

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

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
            "#
            ),
            "\n"
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-mcycle"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_0_s {
    use libaki_mcycle::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_invalid_regex() {
        let (r, sioe) = do_execute!(&["-e", "["]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": regex parse error:\n    [\n    ^\nerror: unclosed character class\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1_s {
    use libaki_mcycle::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    fn make_env() -> conf::EnvConf {
        let mut env = conf::EnvConf::new();
        env.color_seq_red_start = "<R>".to_string();
        env.color_seq_green_start = "<G>".to_string();
        env.color_seq_blue_start = "<B>".to_string();
        env.color_seq_cyan_start = "<C>".to_string();
        env.color_seq_magenda_start = "<M>".to_string();
        env.color_seq_yellow_start = "<Y>".to_string();
        env.color_seq_end = "<E>".to_string();
        env
    }
    //
    #[test]
    fn test_abc_1() {
        let in_put = "abcdefg";
        let env = make_env();
        let (r, sioe) = do_execute!(&env, &["-e", "a"], in_put);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>bcdefg\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_abc_2() {
        let in_put = "abcdefg\nhiajklnm\n";
        let env = make_env();
        let (r, sioe) = do_execute!(&env, &["-e", "a"], in_put);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>bcdefg\nhi<R>a<E>jklnm\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_abc_3() {
        let in_put = "ab1cdefg\nhi2jklnm\nhi1jklnm\n";
        let env = make_env();
        let (r, sioe) = do_execute!(&env, &["-e", "[0-9]"], in_put);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "ab<R>1<E>cdefg\nhi<G>2<E>jklnm\nhi<R>1<E>jklnm\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_abc_4() {
        let in_put = "ab1cdefg\nhi2jklnm\nhi1jklnm\n";
        let env = make_env();
        let (r, sioe) = do_execute!(&env, &["-e", "[a-z][0-9]([a-z])"], in_put);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "ab1<R>c<E>defg\nhi2<G>j<E>klnm\nhi1<G>j<E>klnm\n"
        );
        assert!(r.is_ok());
    }
}

mod test_3_s {
    /*
    use libaki_mcycle::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}
