use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("0.1")
        .author("Talentaa <talentaa@qq.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text required")
        .cloned()
        .collect();

    let omit_newline = matches.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
