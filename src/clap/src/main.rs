use clap::Command;

fn main() {
    // The code is OK start==================================
    // ./clap run --c c:/aaaa
    // let cmd = clap::Command::new("root")
    //     .bin_name("root")
    //     .subcommand_required(true)
    //     .subcommand(
    //         clap::command!("run").arg(
    //             clap::arg!(--"c" <PATH>)
    //                 .required(false)
    //                 .allow_invalid_utf8(true),
    //         ),
    //     )
    //     .subcommand(
    //         clap::command!("check").arg(
    //             clap::arg!(--"c" <PATH>)
    //                 .required(false)
    //                 .allow_invalid_utf8(true),
    //         ),
    //     );
    //
    // let matches = cmd.get_matches();
    // let matches = match matches.subcommand() {
    //     Some(("run", matches)) => matches,
    //     _ => unreachable!("clap should ensure we don't get here"),
    // };
    // let config_path = matches
    //     .value_of_os("c")
    //     .map(std::path::PathBuf::from);
    // println!("{:?}", config_path);
    // The code is OK end==================================

    // The code is error start==================================
    let cmd = clap::Command::new("root")
        .bin_name("root")
        .subcommand_required(true);

    let cmd = init_run(cmd);
    let cmd = init_check(cmd);

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("run", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let config_path = matches.value_of_os("c").map(std::path::PathBuf::from);
    println!("{:?}", config_path);
    // The code is error end==================================
}

pub fn init_run(command: clap::Command) -> Command{
    command.subcommand(
        clap::command!("run")
            .arg(
                clap::arg!(--"c" <PATH>)
                    .required(false)
                    .allow_invalid_utf8(true),
            ),
    )
}

pub fn init_check(command: clap::Command) -> Command{
    command.subcommand(
        clap::command!("check")
            .arg(
                clap::arg!(--"c" <PATH>)
                    .required(false)
                    .allow_invalid_utf8(true),
            )
            .version("0.1.0")
            .help_template("{bin} ({version}) - {usage}"),
    )
}
