use crate::cli::{Cli, Commands};
use clap::Parser;
mod cli;
mod commands;

// The Command enum has one variant per subcommand.
// Each variant holds its args as named fields.
// How does this compare to having one big flat struct
// with all possible args and using `Option` fields
// for things that only apply to some subcommands?
//
// This highlights one of Rust's most famous design
// philosophies: "Make invalid states unrepresentable."
//
// Lets suppose you have struct like this
// struct Cli {
//   command_name: string, // "count", "search"... so on
//   words: Option<bool>, // only valid for "count"
//   pattern: Option<String>, // only valid for "search"
//   case_insensitive: Option<bool>, // only valid for "search"
//
// ===
// The problem with the Flat struc:
// 1. Representable Invalid States: you could have pattern variable populated
// for "count"..
// 2. No Compiler Exhaustiveness: You'd have to write if command == "count" { ... }
//  else if command == "search" { ... }. If you add a new command later, the compiler
//  doesn't warn you if you forget to handle it.
// 3. Memory Bloat
//
// By Using enum, you model the CLI exaclty as it exists in reality: mutually
// exclusive states.
// - Type Safety
// - Exhaustive Routing (through match cli.command)
// - Memory Efficiency: Rust enums take up precisely the memory size of their
//      single largest variant (plus a tiny tag to know which variant it is).
//      It is incredibly memory efficient.

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Count { input, words } => commands::count::run_count(&input, words),
        Commands::Search {
            input,
            pattern,
            case_insensitive,
        } => commands::search::run_search(&input, &pattern, case_insensitive),
        Commands::Stats { input } => commands::stats::run_stats(&input),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        // So, difference between Returning form main (whether returning Result or
        // ExitCode): Rust cleanly closes the main function's scope. Destructors (Drop traits)
        // for variables inside `main` are executed normally. Open network connections are
        // cleanly closed, temp files are deleted, etc.
        // Calling `exit(1)`: This immediately and violently aborts the process at the OS layer.
        // No Drop implementations are run. Any destructors for variables in main (or anywhere up
        // the stack) are completely skipped.
        std::process::exit(1);
    }
}
