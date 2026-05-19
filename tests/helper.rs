#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
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
            "  AKI_MCYCLE_COLOR_SEQ_MAGENTA_ST   magenta start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi\n",
            "  AKI_MCYCLE_COLOR_SEQ_ED           color end sequence specified by ansi\n",
            "\n"
        )
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! x_help_msg {
    () => {
        concat!(
            "Options:\n",
            "  -X rust-version-info     display rust version info and exit\n",
            "\n"
        )
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! env_1 {
    () => {
        vec![
            ("AKI_MCYCLE_COLOR_SEQ_RED_ST", "<R>"),
            ("AKI_MCYCLE_COLOR_SEQ_GREEN_ST", "<G>"),
            ("AKI_MCYCLE_COLOR_SEQ_BLUE_ST", "<B>"),
            ("AKI_MCYCLE_COLOR_SEQ_CYAN_ST", "<C>"),
            ("AKI_MCYCLE_COLOR_SEQ_MAGENTA_ST", "<M>"),
            ("AKI_MCYCLE_COLOR_SEQ_YELLOW_ST", "<Y>"),
            ("AKI_MCYCLE_COLOR_SEQ_ED", "<E>"),
        ]
    };
}
