use clap::Command;

pub fn init() -> Command<'static> {
    let command = clap::Command::new("arana")
        .bin_name("arana")
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
                .about("Start arana Proxy Server"),
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
                .about("Import arana Proxy config"),
        );

    command
}
