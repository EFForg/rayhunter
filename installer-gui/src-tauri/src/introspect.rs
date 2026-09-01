//! Combines the values we care about from the clap CLI and the modifiers module into something
//! serializable by serde.
use std::collections::HashMap;

use anyhow::Context;
use log::error;
use serde::Serialize;

use crate::modifiers;

#[derive(Debug, Serialize)]
pub struct Command<'a> {
    subcommands: Vec<Subcommand<'a>>,
}

impl Command<'_> {
    pub fn new(command: &clap::Command) -> Command<'_> {
        let subcommand_map: HashMap<&str, &clap::Command> = command
            .get_subcommands()
            .map(|s| (s.get_name(), s))
            .collect();

        Command {
            // this resulting vector contains the subcommands that are found in both
            // command.get_subcommands() and modifiers::subcommand_modifiers() in the order defined
            // by subcommand_modifiers()
            subcommands: modifiers::subcommand_modifiers()
                .iter()
                .filter_map(|modifier| match subcommand_map.get(modifier.command) {
                    Some(clap_command) => Some(Subcommand::new(clap_command, modifier)),
                    None => {
                        error!(
                            "failed to find clap command for subcommand {}",
                            modifier.command
                        );
                        None
                    }
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Argument<'a> {
    advanced: bool,
    flag: String,
    help: String,
    label: &'a str,
    takes_values: bool,
}

#[derive(Debug, Serialize)]
struct Subcommand<'a> {
    arguments: Vec<Argument<'a>>,
    command: &'a str,
    label: &'a str,
}

fn argument_help(argument: &clap::Arg) -> String {
    let mut help = argument
        .get_help()
        .map(ToString::to_string)
        .unwrap_or_default();
    let default_values = argument.get_default_values();

    if !argument.is_hide_default_value_set() && !default_values.is_empty() {
        if !help.is_empty() {
            help.push(' ');
        }
        help.push_str("[default: ");
        help.push_str(
            &default_values
                .iter()
                .map(|value| value.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
        );
        help.push(']');
    }

    help
}

impl Argument<'_> {
    fn try_new<'a>(
        argument: &'a clap::Arg,
        modifier: &modifiers::ArgumentModifier<'static>,
    ) -> anyhow::Result<Argument<'a>> {
        let partial_flag = argument.get_long().with_context(|| {
            format!(
                "Missing long form command line flag for {}",
                argument.get_id().as_str(),
            )
        })?;
        Ok(Argument {
            advanced: modifier.advanced,
            flag: format!("--{}", partial_flag),
            help: argument_help(argument),
            label: modifier.gui_label,
            takes_values: argument.get_action().takes_values(),
        })
    }
}

impl Subcommand<'_> {
    fn new<'a>(
        command: &'a clap::Command,
        modifier: &modifiers::SubcommandModifier<'static>,
    ) -> Subcommand<'a> {
        let argument_map: HashMap<&str, &clap::Arg> = command
            .get_arguments()
            .map(|a| (a.get_id().as_str(), a))
            .collect();

        Subcommand {
            // this resulting vector contains the arguments that are found in both
            // command.get_arguments() and modifier.arg_modifiers in the order defined by by
            // arg_modifiers
            arguments: modifier
                .arg_modifiers
                .iter()
                .filter_map(|arg_modifier| {
                    let Some(arg) = argument_map.get(arg_modifier.clap_id) else {
                        error!(
                            "failed to find clap argument with id {}",
                            arg_modifier.clap_id
                        );
                        return None;
                    };
                    match Argument::try_new(arg, arg_modifier) {
                        Ok(modified_arg) => Some(modified_arg),
                        Err(err) => {
                            error!("failed to create modified argument: {:?}", err);
                            None
                        }
                    }
                })
                .collect(),
            command: modifier.command,
            label: modifier.gui_label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::argument_help;

    #[test]
    fn argument_help_includes_default_values() {
        let argument = clap::Arg::new("example")
            .help("Choose an example value")
            .default_values(["one", "two"]);

        assert_eq!(
            argument_help(&argument),
            "Choose an example value [default: one two]"
        );
    }

    #[test]
    fn argument_help_respects_hidden_default_values() {
        let argument = clap::Arg::new("example")
            .help("Choose an example value")
            .default_value("one")
            .hide_default_value(true);

        assert_eq!(argument_help(&argument), "Choose an example value");
    }
}
