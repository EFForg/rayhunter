//! Adds or "modifies" installer CLI attributes for use in the GUI.
//!
//! This module contains little logic (outside of tests) and instead just provides additional
//! metadata about CLI commands and options for the GUI installer.
//!
//! If we like this approach, I think we should consider renaming this file something like
//! gui_modifiers.rs and moving it into the crate for the CLI installer. I think this would simplify
//! development as any breaking changes to the CLI installer interface would cause tests to fail in
//! its own crate instead of installer-gui and it'd help to keep the two interfaces to the installer
//! in sync.

#[derive(Debug, Copy, Clone)]
pub struct ArgumentModifier<'a> {
    /// The name or "ID" of the argument as defined in clap. This will usually be the name of the
    /// field in the struct the argument is derived from.
    pub clap_id: &'a str,
    /// The text for displaying this argument in the GUI.
    pub gui_label: &'a str,
    /// Whether this argument should be hidden behind a menu for "advanced" options.
    pub advanced: bool,
}

#[derive(Debug)]
pub struct SubcommandModifier<'a> {
    /// The name of the subcommand on the CLI.
    pub command: &'a str,
    /// The text for displaying this subcommand in the GUI.
    pub gui_label: &'a str,
    /// Modifications to the arguments of this subcommand. The order arguments are defined in this
    /// vector will match the order the arguments are displayed in the GUI.
    pub arg_modifiers: Vec<ArgumentModifier<'a>>,
}

/// Provides "modifiers" or additional metadata about each subcommand.
///
/// The order of the subcommands in the returned vector is the same order that subcommands will be
/// shown in the GUI. Similarly, the order of the elements in each arg_modifiers field controls the
/// order that a subcommand's options will be shown in the GUI.
pub fn subcommand_modifiers() -> Vec<SubcommandModifier<'static>> {
    let admin_ip = ArgumentModifier {
        clap_id: "admin_ip",
        gui_label: "Admin IP",
        advanced: true,
    };
    let admin_username = ArgumentModifier {
        clap_id: "admin_username",
        gui_label: "Admin Username",
        advanced: true,
    };
    let admin_password = ArgumentModifier {
        clap_id: "admin_password",
        gui_label: "Admin Password",
        advanced: false,
    };
    let data_dir = ArgumentModifier {
        clap_id: "data_dir",
        gui_label: "Data Directory",
        advanced: true,
    };
    let reset_config = ArgumentModifier {
        clap_id: "reset_config",
        gui_label: "Reset config.toml",
        advanced: true,
    };
    let orbic_and_moxee_args = vec![
        admin_password,
        admin_ip,
        admin_username,
        reset_config,
        data_dir,
    ];

    vec![
        SubcommandModifier {
            command: "orbic",
            gui_label: "Orbic/Kajeet (via network)",
            arg_modifiers: orbic_and_moxee_args.clone(),
        },
        SubcommandModifier {
            command: "orbic-usb",
            gui_label: "Orbic/Kajeet (via legacy USB+ADB installer)",
            arg_modifiers: vec![reset_config],
        },
        SubcommandModifier {
            command: "tplink",
            gui_label: "TP-Link",
            arg_modifiers: vec![
                admin_ip,
                reset_config,
                data_dir,
                ArgumentModifier {
                    clap_id: "skip_sdcard",
                    gui_label: "Skip SD Card",
                    advanced: true,
                },
                ArgumentModifier {
                    clap_id: "sdcard_path",
                    gui_label: "SD Card Path",
                    advanced: true,
                },
            ],
        },
        SubcommandModifier {
            command: "moxee",
            gui_label: "Moxee",
            arg_modifiers: orbic_and_moxee_args,
        },
        SubcommandModifier {
            command: "pinephone",
            gui_label: "PinePhone",
            arg_modifiers: vec![],
        },
        SubcommandModifier {
            command: "tmobile",
            gui_label: "TMobile",
            arg_modifiers: vec![admin_password, admin_ip],
        },
        SubcommandModifier {
            command: "uz801",
            gui_label: "UZ801",
            arg_modifiers: vec![admin_ip],
        },
        SubcommandModifier {
            command: "wingtech",
            gui_label: "Wingtech",
            arg_modifiers: vec![admin_password, admin_ip],
        },
    ]
}

#[cfg(test)]
mod tests {
    //! Subcommands and arguments not returned from subcommand_modifiers() will be excluded from the
    //! GUI. This is by design as it allows us to exclude things like some or all of the installer
    //! utils from the GUI. The tests below help ensure that exclusions were done deliberately
    //! rather than on accident.
    use super::*;
    use std::collections::HashMap;

    /// Lists the subcommands that are purposefully excluded from subcommand_modifiers().
    fn excluded_subcommands() -> Vec<&'static str> {
        vec!["util"]
    }

    /// Lists the arguments that are purposefully excluded from subcommand_modifiers(). Items in the
    /// list take the form of (subcommand, argument_id) tuples.
    fn excluded_arguments() -> Vec<(&'static str, &'static str)> {
        // if for example we wanted to exclude the "--admin-password" argument for "orbic", we'd
        // return vec![("orbic", "admin_password")] here
        vec![]
    }

    #[test]
    fn test_subcommands_excluded_or_modified() {
        let mut all_subcommands: Vec<&str> = crate::INSTALLER_COMMAND
            .get_subcommands()
            .map(|c| c.get_name())
            .collect();
        let mut excluded_or_modified_subcommands: Vec<&str> = subcommand_modifiers()
            .into_iter()
            .map(|m| m.command)
            .chain(excluded_subcommands())
            .collect();

        all_subcommands.sort_unstable();
        excluded_or_modified_subcommands.sort_unstable();

        assert_eq!(
            all_subcommands, excluded_or_modified_subcommands,
            "Every subcommand must be included exactly once in subcommand_modifiers() or excluded_subcommands()."
        );
    }

    #[test]
    fn test_arguments_excluded_or_modified() {
        // create maps of subcommand name to lists of argument names
        let all_args_for_nonexcluded_subcommands: HashMap<&str, Vec<&str>> =
            nonexcluded_subcommand_objects()
                .into_iter()
                .map(|c| {
                    (
                        c.get_name(),
                        c.get_arguments().map(|a| a.get_id().as_str()).collect(),
                    )
                })
                .collect();
        let modified_args: HashMap<&str, Vec<&str>> = subcommand_modifiers()
            .into_iter()
            .map(|m| {
                (
                    m.command,
                    m.arg_modifiers
                        .into_iter()
                        .map(|arg_m| arg_m.clap_id)
                        .collect(),
                )
            })
            .collect();

        // add excluded_arguments to modified_args
        let mut excluded_or_modified_args = modified_args;
        for (subcommand_name, arg_name) in excluded_arguments() {
            excluded_or_modified_args
                .entry(subcommand_name)
                .or_default()
                .push(arg_name);
        }

        // assert that all arguments are excluded or modified
        for (subcommand_name, mut expected_args) in all_args_for_nonexcluded_subcommands {
            let mut found_args = excluded_or_modified_args
                .remove(subcommand_name)
                .unwrap_or_default();

            expected_args.sort_unstable();
            found_args.sort_unstable();

            assert_eq!(
                expected_args, found_args,
                "Excluded and modified arguments differ from expected arguments for {subcommand_name}."
            )
        }
        assert!(
            excluded_or_modified_args.is_empty(),
            "Excluded or modified arguments found for unexpected subcommands. Map of unexpected arguments is {:?}",
            excluded_or_modified_args
        );
    }

    #[test]
    fn test_arguments_have_long_flag() {
        // any arguments without a long form command line flag will be excluded from the GUI so
        // let's test for it here to avoid surprises

        let excluded_args = excluded_arguments();
        let nonexcluded_args: Vec<(&str, &clap::Arg)> = nonexcluded_subcommand_objects()
            .into_iter()
            .flat_map(|c| {
                c.get_arguments().filter_map(|a| {
                    let subcommand_name = c.get_name();

                    if excluded_args.contains(&(subcommand_name, a.get_id().as_str())) {
                        None
                    } else {
                        Some((subcommand_name, a))
                    }
                })
            })
            .collect();

        for (subcommand_name, arg) in nonexcluded_args {
            assert!(
                arg.get_long().is_some(),
                "The {} argument for {subcommand_name} is missing a long form command line flag.",
                arg.get_id().as_str()
            )
        }
    }

    fn nonexcluded_subcommand_objects() -> Vec<&'static clap::Command> {
        let excluded_subcommands = excluded_subcommands();

        crate::INSTALLER_COMMAND
            .get_subcommands()
            .filter(|s| !excluded_subcommands.contains(&s.get_name()))
            .collect()
    }
}
