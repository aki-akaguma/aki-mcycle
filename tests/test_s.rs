macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-mcycle [options]\n",
            "\n",
            "mark up text with cycling color.\n",
            "\n",
            "Options:\n",
            "  -e, --exp <exp>      regex (default: ' ([0-9A-Z]{3,}):')\n",
            "\n",
            "  -H, --help     display this help and exit\n",
            "  -V, --version  display version information and exit\n",
            "\n",
            "Env:\n",
            "  RUST_CYCLE_COLOR_RED_ST      red start sequence\n",
            "  RUST_CYCLE_COLOR_GREEN_ST    green start sequence\n",
            "  RUST_CYCLE_COLOR_BLUE_ST     blue start sequence\n",
            "  RUST_CYCLE_COLOR_CYAN_ST     cyan start sequence\n",
            "  RUST_CYCLE_COLOR_MAGENDA_ST  magenda start sequence\n",
            "  RUST_CYCLE_COLOR_YELLOW_ST   yellow start sequence\n",
            "  RUST_CYCLE_COLOR_ED          color end sequence\n",
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
        let sioe = StreamIoe {
            sin: Box::new(StreamInStringIn::with_str($sin)),
            sout: Box::new(StreamOutStringOut::default()),
            serr: Box::new(StreamErrStringErr::default()),
        };
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .serr
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.serr.lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.sout.lock().buffer_str()
    };
}

mod test_0 {
    use libaki_mcycle::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
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
        assert_eq!(r.is_ok(), false);
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
        assert_eq!(r.is_ok(), false);
    }
}

mod test_1 {
    /*
        use runnel::medium::stringio::*;
        use runnel::*;
        use libaki_mcycle::*;
        use std::io::Write;
        use std::collections::HashMap;
        //
        fn make_env() -> HashMap<String, String> {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_CYCLE_COLOR_RED_ST".to_string(), "<R>".to_string());
            env.insert("RUST_CYCLE_COLOR_GREEN_ST".to_string(), "<G>".to_string());
            env.insert("RUST_CYCLE_COLOR_BLUE_ST".to_string(), "<B>".to_string());
            env.insert("RUST_CYCLE_COLOR_CYAN_ST".to_string(), "<C>".to_string());
            env.insert("RUST_CYCLE_COLOR_MAGENDA_ST".to_string(), "<M>".to_string());
            env.insert("RUST_CYCLE_COLOR_YELLOW_ST".to_string(), "<Y>".to_string());
            env.insert("RUST_CYCLE_COLOR_ED".to_string(), "<E>".to_string());
            env
        }
        //
        #[test]
        fn test_abc_1() {
            let in_put: &[u8] = b"abcdefg";
            let env: HashMap<String, String> = make_env();
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-e", "a"], env, in_put);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "<R>a<E>bcdefg\n");
            assert_eq!(oup.status.success(), true);
        }
        //
        #[test]
        fn test_abc_2() {
            let in_put: &[u8] = b"abcdefg\nhiajklnm\n";
            let env: HashMap<String, String> = make_env();
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-e", "a"], env, in_put);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "<R>a<E>bcdefg\nhi<R>a<E>jklnm\n");
            assert_eq!(oup.status.success(), true);
        }
        //
        #[test]
        fn test_abc_3() {
            let in_put: &[u8] = b"ab1cdefg\nhi2jklnm\nhi1jklnm\n";
            let env: HashMap<String, String> = make_env();
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-e", "[0-9]"], env, in_put);
            assert_eq!(oup.stderr, "");
            assert_eq!(
                oup.stdout,
                "ab<R>1<E>cdefg\nhi<G>2<E>jklnm\nhi<R>1<E>jklnm\n"
            );
            assert_eq!(oup.status.success(), true);
        }
        //
        #[test]
        fn test_abc_4() {
            let in_put: &[u8] = b"ab1cdefg\nhi2jklnm\nhi1jklnm\n";
            let env: HashMap<String, String> = make_env();
            let oup =
                exec_target_with_env_in(TARGET_EXE_PATH, &["-e", "[a-z][0-9]([a-z])"], env, in_put);
            assert_eq!(oup.stderr, "");
            assert_eq!(
                oup.stdout,
                "ab1<R>c<E>defg\nhi2<G>j<E>klnm\nhi1<G>j<E>klnm\n"
            );
            assert_eq!(oup.status.success(), true);
        }
    */
}

mod test_3 {
    /*
    use streamio::stringio::*;
    use libaki_mcycle::*;
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}
