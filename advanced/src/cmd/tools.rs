use clap::Command;

pub fn init(command: clap::Command) -> Command {
    command.subcommand(
        clap::command!("import")
            .arg(
                clap::arg!(--"c" <PATH>)
                    .required(false)
                    .allow_invalid_utf8(true),
            )
            .version("0.1.0")
            .help_template("{bin} ({version}) - {usage} {all-args} {about}")
            .about("tttttttttttttttttttttt"),
    )
}
