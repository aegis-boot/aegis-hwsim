//! `aegis-hwsim` CLI skeleton. Real orchestration lands in Phase 1 per
//! the plan in aegis-boot#226; this file exists so the repo builds and
//! the subcommand surface is reviewable.

#![forbid(unsafe_code)]

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("list-personas") => {
            eprintln!("aegis-hwsim: list-personas — not implemented yet");
            eprintln!("  Will scan personas/*.yaml and print id + display_name.");
            ExitCode::from(3)
        }
        Some("validate") => {
            eprintln!("aegis-hwsim: validate — not implemented yet");
            eprintln!("  Will parse personas/*.yaml against the schema and report errors.");
            ExitCode::from(3)
        }
        Some("run") => {
            eprintln!("aegis-hwsim: run — not implemented yet");
            eprintln!("  Usage: aegis-hwsim run <persona> <scenario> <aegis-boot-stick.img>");
            ExitCode::from(3)
        }
        Some("-h" | "--help" | "help") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("--version" | "version") => {
            println!("aegis-hwsim v{}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("aegis-hwsim: unknown subcommand '{other}'");
            eprintln!("run 'aegis-hwsim --help' for usage");
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!("aegis-hwsim — hardware-persona matrix harness for aegis-boot");
    println!();
    println!("USAGE:");
    println!("  aegis-hwsim list-personas           List YAML fixtures under personas/");
    println!("  aegis-hwsim validate                Validate all personas against the schema");
    println!("  aegis-hwsim run <persona> <scenario> <stick>");
    println!("                                      Boot the stick under the named persona");
    println!("                                      and run the named scenario against it");
    println!("  aegis-hwsim --version               Print version");
    println!("  aegis-hwsim --help                  This message");
    println!();
    println!("STATUS:");
    println!("  Scaffolding phase. Subcommands exit 3 until implementation lands.");
    println!("  Track progress: https://github.com/williamzujkowski/aegis-boot/issues/226");
}
