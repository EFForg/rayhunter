//! Combines the values we care about from the clap CLI and the modifiers module into something
//! serializable by serde.
use std::collections::HashMap;

use anyhow::Context;
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
                .filter_map(|modifier| {
                    subcommand_map
                        .get(modifier.command)
                        .map(|subcommand| Subcommand::new(subcommand, modifier))
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Argument<'a> {
    advanced: bool,
    flag: String,
    label: &'a str,
    takes_values: bool,
}

#[derive(Debug, Serialize)]
struct Subcommand<'a> {
    arguments: Vec<Argument<'a>>,
    command: &'a str,
    label: &'a str,
}

impl Argument<'_> {
    fn try_new<'a>(
        argument: &'a clap::Arg,
        modifier: &modifiers::ArgumentModifier<'static>,
    ) -> anyhow::Result<Argument<'a>> {
        // if an argument doesn't have the data we need, it's silently dropped from the GUI, however,
        // tests should prevent this from happening and we could add logging messages about this in
        // the future if desired
        let partial_flag = argument.get_long().with_context(|| {
            format!(
                "Missing long form command line flag for {}",
                argument.get_id().as_str(),
            )
        })?;
        Ok(Argument {
            advanced: modifier.advanced,
            flag: format!("--{}", partial_flag),
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
                    argument_map
                        .get(arg_modifier.clap_id)
                        .and_then(|arg| Argument::try_new(arg, arg_modifier).ok())
                })
                .collect(),
            command: modifier.command,
            label: modifier.gui_label,
        }
    }
}
