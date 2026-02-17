mod codegen;
mod graph;
mod mermaid;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Parser, Subcommand};

use graph::DepGraph;

#[derive(Parser)]
#[command(name = "rayhunter-deps", about = "Rayhunter dependency graph tool")]
struct Cli {
    #[arg(long, global = true)]
    manifest: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check manifest for errors
    Validate,
    /// Show what breaks if a feature is removed
    Impact { name: String },
    /// Show transitive dependencies of a feature
    Deps { name: String },
    /// Show all features for a device
    Device { name: String },
    /// Output Mermaid diagram
    Graph {
        #[arg(long)]
        device: Option<String>,
        #[arg(long, value_parser = ["maintainer"])]
        color_by: Option<String>,
    },
    /// Generate Rust device capability map
    Generate {
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        check: bool,
    },
}

fn find_manifest(override_path: Option<&PathBuf>) -> PathBuf {
    if let Some(p) = override_path {
        return p.clone();
    }
    let exe = std::env::current_exe().unwrap_or_default();
    let mut dir = exe.parent().map(|p| p.to_path_buf());
    while let Some(d) = dir {
        let candidate = d.join("deps.toml");
        if candidate.exists() {
            return candidate;
        }
        dir = d.parent().map(|p| p.to_path_buf());
    }

    let cwd = std::env::current_dir().unwrap_or_default();
    let mut dir = Some(cwd.as_path());
    while let Some(d) = dir {
        let candidate = d.join("deps.toml");
        if candidate.exists() {
            return candidate;
        }
        dir = d.parent();
    }

    eprintln!("Cannot find deps.toml");
    process::exit(1);
}

fn load_graph(manifest_override: Option<&PathBuf>) -> DepGraph {
    let deps_path = find_manifest(manifest_override);
    let maint_path = deps_path.with_file_name("maintenance.toml");
    match DepGraph::load(&deps_path, &maint_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}

fn cmd_validate(graph: &DepGraph) {
    let errors = graph.validate();
    if errors.is_empty() {
        let maint_note = if graph.maintenance.is_empty() {
            String::new()
        } else {
            format!(", {} maintenance entries", graph.maintenance.len())
        };
        println!(
            "Valid. {} features, {} devices{maint_note}.",
            graph.features.len(),
            graph.devices.len()
        );
    } else {
        println!("Found {} error(s):", errors.len());
        println!();
        for e in &errors {
            println!("  - {e}");
        }
        process::exit(1);
    }
}

fn cmd_impact(graph: &DepGraph, name: &str) {
    let Some(feat) = graph.features.get(name) else {
        eprintln!("Unknown feature: {name}");
        process::exit(1);
    };

    let dependents = graph.transitive_dependents(name, true);
    let devices = graph.affected_devices(name);

    println!("Impact of removing '{name}':");
    println!("  {}", feat.description);
    println!();

    if dependents.is_empty() {
        println!("  No other features depend on this.");
    } else {
        println!("  Features that would break:");
        let mut sorted: Vec<&str> = dependents.iter().map(|s| s.as_str()).collect();
        sorted.sort();
        for d in sorted {
            if let Some(f) = graph.features.get(d) {
                println!("    - {d}: {}", f.description);
            }
        }
    }

    println!();
    println!(
        "  Affected devices: {}",
        devices.into_iter().collect::<Vec<_>>().join(", ")
    );

    if let Some(m) = graph.maintenance.get(name) {
        println!();
        println!(
            "  Maintainer: {}",
            m.maintainer.as_deref().unwrap_or("unknown")
        );
        if let Some(intro) = &m.introduced_by {
            print!(
                "  Introduced by: {}",
                intro.author.as_deref().unwrap_or("unknown")
            );
            if let Some(pr) = intro.pr {
                print!(" (PR #{pr})");
            }
            if let Some(date) = &intro.date {
                print!(" on {date}");
            }
            println!();
        }
    }
}

fn cmd_deps(graph: &DepGraph, name: &str) {
    let Some(feat) = graph.features.get(name) else {
        eprintln!("Unknown feature: {name}");
        process::exit(1);
    };

    let deps = graph.transitive_deps(name, false);
    let all_deps = graph.transitive_deps(name, true);
    let runtime_only: Vec<&str> = all_deps.difference(&deps).map(|s| s.as_str()).collect();

    println!("Dependencies of '{name}':");
    println!("  {}", feat.description);
    println!();

    let direct = if feat.depends_on.is_empty() {
        "(none)".to_string()
    } else {
        let mut d = feat.depends_on.clone();
        d.sort();
        d.join(", ")
    };
    let runtime = if feat.runtime_deps.is_empty() {
        "(none)".to_string()
    } else {
        let mut r = feat.runtime_deps.clone();
        r.sort();
        r.join(", ")
    };
    let all = if all_deps.is_empty() {
        "(none)".to_string()
    } else {
        let mut combined: Vec<&str> = all_deps.iter().map(|s| s.as_str()).collect();
        combined.sort();
        combined.join(", ")
    };

    println!("  Direct: {direct}");
    println!("  Runtime: {runtime}");
    let _ = runtime_only;
    println!("  All transitive: {all}");
}

fn cmd_device(graph: &DepGraph, name: &str) {
    if !graph.devices.contains_key(name) {
        eprintln!("Unknown device: {name}");
        process::exit(1);
    }

    let features = graph.features_for_device(name);
    let mut categories: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for fname in &features {
        if let Some(feat) = graph.features.get(fname.as_str()) {
            categories
                .entry(feat.category.as_str())
                .or_default()
                .push(fname.as_str());
        }
    }

    let desc = &graph.devices[name].description;
    println!("Features for device '{name}' ({desc}):");
    println!();
    for (cat, names) in &categories {
        println!("  [{cat}]");
        for &fname in names {
            let mut maint_tag = String::new();
            if let Some(m) = graph.maintenance.get(fname) {
                match m.maintainer.as_deref() {
                    Some("unmaintained") => maint_tag = " [UNMAINTAINED]".to_string(),
                    Some(s) if s.starts_with("community:") => maint_tag = format!(" [{s}]"),
                    _ => {}
                }
            }
            println!("    - {fname}{maint_tag}");
        }
        println!();
    }
}

fn cmd_graph(graph: &DepGraph, device: Option<&str>, color_by: Option<&str>) {
    println!("{}", graph.to_mermaid(device, color_by));
}

fn cmd_generate(graph: &DepGraph, output: Option<&PathBuf>, check: bool, deps_path: &Path) {
    let content = graph.generate_rust_capabilities();

    let output_path = match output {
        Some(p) => p.clone(),
        None => deps_path
            .parent()
            .unwrap()
            .join("daemon")
            .join("src")
            .join("device_capabilities.rs"),
    };

    if check {
        if !output_path.exists() {
            println!(
                "{} does not exist, run 'rayhunter-deps generate' first",
                output_path.display()
            );
            process::exit(1);
        }
        let existing = std::fs::read_to_string(&output_path).unwrap();
        if existing == content {
            println!(
                "{} is up to date.",
                output_path.file_name().unwrap().to_str().unwrap()
            );
        } else {
            println!(
                "{} is out of date, run 'rayhunter-deps generate'",
                output_path.file_name().unwrap().to_str().unwrap()
            );
            process::exit(1);
        }
    } else {
        std::fs::write(&output_path, &content).unwrap();
        let cap_count = graph.capability_features().len();
        println!(
            "Generated {} ({cap_count} capabilities, {} devices)",
            output_path.display(),
            graph.devices.len()
        );
    }
}

fn main() {
    let cli = Cli::parse();
    let deps_path = find_manifest(cli.manifest.as_ref());
    let graph = load_graph(cli.manifest.as_ref());

    match &cli.command {
        Command::Validate => cmd_validate(&graph),
        Command::Impact { name } => cmd_impact(&graph, name),
        Command::Deps { name } => cmd_deps(&graph, name),
        Command::Device { name } => cmd_device(&graph, name),
        Command::Graph { device, color_by } => {
            cmd_graph(&graph, device.as_deref(), color_by.as_deref())
        }
        Command::Generate { output, check } => {
            cmd_generate(&graph, output.as_ref(), *check, &deps_path)
        }
    }
}
