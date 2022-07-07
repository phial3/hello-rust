use clap::Command;

pub fn init() -> Command<'static> {
    let command = clap::Command::new("mydbproxy")
        .bin_name("mydbproxy")
        .subcommand_required(true)
        .subcommand(
            clap::command!("start")
                .arg(
                    clap::arg!(--"c" <PATH>)
                        .required(false)
                        .allow_invalid_utf8(true),
                )
                .version("0.1.0")
                .help_template("{bin} ({version}) - {usage}")
                .about("Start Proxy Server"),
        )
        .subcommand(
            clap::command!("import")
                .arg(
                    clap::arg!(--"c" <PATH>)
                        .required(false)
                        .allow_invalid_utf8(true),
                )
                .version("0.1.0")
                .help_template("{bin} ({version}) - {usage} {all-args} {about}")
                .about("Import Proxy config"),
        ).help_expected(true);

    command
}
