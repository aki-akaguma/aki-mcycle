const TARGET_EXE_PATH: &'static str = "target/debug/aki-mcycle";

macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-mcycle [options]\n",
            "\n",
            "mark up text with the cyclic color.\n",
            "\n",
            "Options:\n",
            "  -e, --exp <exp>   write it in the cyclic color (default: ' ([0-9A-Z]{3,}):')\n",
            "\n",
            "  -H, --help        display this help and exit\n",
            "  -V, --version     display version information and exit\n",
            "  -X <x-options>    x options. try -X help\n",
            "\n",
            "Option Parameters:\n",
            "  <exp>     regular expression, color the entire match with the cyclic color.\n",
            "\n",
            "Environments:\n",
            "  AKI_MCYCLE_COLOR_SEQ_RED_ST       red start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_GREEN_ST     green start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_ED           color end sequence specified by ansi\n",
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

macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

mod helper;

mod test_0 {
    use crate::helper::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, &[""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    #[test]
    fn test_invalid_regex() {
        let oup = exec_target(TARGET_EXE_PATH, &["-e", "["]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": regex parse error:\n    [\n    ^\nerror: unclosed character class\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
}

mod test_1 {
    use crate::helper::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    use std::collections::HashMap;
    //
    fn make_env() -> HashMap<String, String> {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("AKI_MCYCLE_COLOR_SEQ_RED_ST".to_string(), "<R>".to_string());
        env.insert(
            "AKI_MCYCLE_COLOR_SEQ_GREEN_ST".to_string(),
            "<G>".to_string(),
        );
        env.insert(
            "AKI_MCYCLE_COLOR_SEQ_BLUE_ST".to_string(),
            "<B>".to_string(),
        );
        env.insert(
            "AKI_MCYCLE_COLOR_SEQ_CYAN_ST".to_string(),
            "<C>".to_string(),
        );
        env.insert(
            "AKI_MCYCLE_COLOR_SEQ_MAGENDA_ST".to_string(),
            "<M>".to_string(),
        );
        env.insert(
            "AKI_MCYCLE_COLOR_SEQ_YELLOW_ST".to_string(),
            "<Y>".to_string(),
        );
        env.insert("AKI_MCYCLE_COLOR_SEQ_ED".to_string(), "<E>".to_string());
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
}

mod test_3 {
    use crate::helper::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -e \"A\" | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", &["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\u{1b}[31mA\u{1b}[0mBCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
    }
}
