use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Command<'a> {
    subcommands: Vec<Subcommand<'a>>,
}

impl Command<'_> {
    pub fn new(clap_command: &clap::Command) -> Command<'_> {
        Command {
            subcommands: clap_command
                .get_subcommands()
                // at least for now, we filter out subcommands that themselves have subcommands like
                // "util" as supporting these would require additional changes to both the frontend
                // and this module
                .filter(|c| !c.has_subcommands())
                .map(|c| Subcommand::new(c))
                .collect(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Argument<'a> {
    name: &'a str,
    takes_values: bool,
}

#[derive(Serialize, Debug)]
struct Subcommand<'a> {
    arguments: Vec<Argument<'a>>,
    name: &'a str,
}

impl Argument<'_> {
    fn new(clap_argument: &clap::Arg) -> Argument<'_> {
        Argument {
            name: clap_argument.get_id().as_str(),
            takes_values: clap_argument.get_action().takes_values(),
        }
    }
}

impl Subcommand<'_> {
    fn new(clap_command: &clap::Command) -> Subcommand<'_> {
        Subcommand {
            arguments: clap_command
                .get_arguments()
                .map(|a| Argument::new(a))
                .collect(),
            name: clap_command.get_name(),
        }
    }
}
