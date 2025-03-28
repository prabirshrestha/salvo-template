use argh::FromArgs;
use std::process::{Command, ExitStatus};

/// Tasks for the salvo-template project
#[derive(FromArgs)]
struct Cli {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Commands {
    Build(BuildArgs),
    Test(TestArgs),
    Lint(LintArgs),
}

/// Build the project
#[derive(FromArgs)]
#[argh(subcommand, name = "build")]
struct BuildArgs {
    /// build with release profile
    #[argh(switch)]
    release: bool,

    /// build with all features
    #[argh(switch)]
    all_features: bool,
}

/// Run tests
#[derive(FromArgs)]
#[argh(subcommand, name = "test")]
struct TestArgs {
    /// test with release profile
    #[argh(switch)]
    release: bool,

    /// test with all features
    #[argh(switch)]
    all_features: bool,
}

/// Run linting checks
#[derive(FromArgs)]
#[argh(subcommand, name = "lint")]
struct LintArgs {
    /// check formatting only without auto-fixing
    #[argh(switch)]
    check: bool,
}

fn main() {
    let cli: Cli = argh::from_env();

    let result = match cli.command {
        Commands::Build(args) => build(args),
        Commands::Test(args) => test(args),
        Commands::Lint(args) => lint(args),
    };

    if !result.success() {
        std::process::exit(result.code().unwrap_or(1));
    }
}

fn build(args: BuildArgs) -> ExitStatus {
    let cargo = env("CARGO", "cargo");
    let mut cmd = Command::new(cargo);

    cmd.arg("build");

    if args.release {
        cmd.arg("--release");
    }

    if args.all_features {
        cmd.arg("--all-features");
    }

    cmd.arg("--all");

    println!("Building project...");
    cmd.status().expect("failed to execute build")
}

fn test(args: TestArgs) -> ExitStatus {
    let cargo = env("CARGO", "cargo");
    let mut cmd = Command::new(cargo);

    cmd.arg("test");

    if args.release {
        cmd.arg("--release");
    }

    if args.all_features {
        cmd.arg("--all-features");
    }

    println!("Running tests...");
    cmd.status().expect("failed to execute tests")
}

fn lint(args: LintArgs) -> ExitStatus {
    let cargo = env("CARGO", "cargo");
    let mut fmt_cmd = Command::new(&cargo);

    fmt_cmd.arg("fmt");

    if args.check {
        fmt_cmd.arg("--all").arg("--").arg("--check");
        println!("Checking code formatting...");
    } else {
        fmt_cmd.arg("--all");
        println!("Formatting code...");
    }

    let fmt_result = fmt_cmd.status().expect("failed to execute fmt");
    if !fmt_result.success() {
        return fmt_result;
    }

    let mut clippy_cmd = Command::new(&cargo);
    clippy_cmd
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features");

    println!("Running clippy...");
    clippy_cmd.status().expect("failed to execute clippy")
}

fn env(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}
