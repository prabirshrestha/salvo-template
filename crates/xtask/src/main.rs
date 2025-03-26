use argh::FromArgs;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, exit},
};

#[derive(FromArgs)]
/// xtasks for salvo-template
struct CmdArgs {
    #[argh(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Build(BuildArgs),
    Dev(DevArgs),
    Tailwind(TailwindArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Build the project
#[argh(subcommand, name = "build")]
struct BuildArgs {
    /// build with release profile
    #[argh(switch)]
    release: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Run the development server
#[argh(subcommand, name = "dev")]
struct DevArgs {}

#[derive(FromArgs, PartialEq, Debug)]
/// Run Tailwind CSS operations
#[argh(subcommand, name = "tailwind")]
struct TailwindArgs {
    /// watch for changes
    #[argh(switch)]
    watch: bool,

    /// build for production
    #[argh(switch)]
    build: bool,
}

impl CmdArgs {
    fn from_env() -> Self {
        argh::from_env()
    }
}

/// Download a file from a URL to a destination path
///
/// # Arguments
/// * `url` - The URL to download from
/// * `destination` - The path where the file should be saved
/// * `make_executable` - Whether to make the file executable (Unix only)
///
/// # Returns
/// The path to the downloaded file
async fn download_file(
    url: &str,
    destination: &Path,
    make_executable: bool,
) -> std::io::Result<PathBuf> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    if destination.exists() {
        if make_executable {
            make_file_executable(destination)?;
        }
        return Ok(destination.to_path_buf());
    }

    println!("Downloading from {} to {}", url, destination.display());

    let temp_path = destination.with_extension("download");

    if temp_path.exists() {
        fs::remove_file(&temp_path)?;
    }

    // ls on destination path
    println!(
        "Listing files in destination path: {}",
        destination.parent().unwrap().display()
    );
    let entries = fs::read_dir(destination.parent().unwrap())?;
    for entry in entries {
        println!("{}", entry?.path().display());
    }

    let client = Client::new();
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let total_size = res.content_length().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to get content length")
    })?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .expect("Invalid progress bar template")
        .progress_chars("#>-"));

    let mut file = File::create(&temp_path)?;
    let mut stream = res.bytes_stream();

    let mut downloaded: u64 = 0;
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        file.write_all(&chunk)?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    file.flush()?;
    drop(file);

    fs::rename(&temp_path, destination)?;

    pb.finish_with_message(format!("Downloaded to {}", destination.display()));

    if make_executable {
        make_file_executable(destination)?;
    }

    Ok(destination.to_path_buf())
}

/// Make a file executable (Unix only)
fn make_file_executable(path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }
    Ok(())
}

async fn run_build(args: &BuildArgs) {
    // First, run Tailwind CSS in build mode (not watch mode)
    let tailwind_args = TailwindArgs {
        watch: false,
        build: true,
    };

    println!("Building CSS with Tailwind...");
    run_tailwind(&tailwind_args).await;
    println!("Tailwind CSS build complete.");

    // Then run cargo build
    println!("Building Rust project...");
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if args.release {
        cmd.arg("--release");
    }

    let status = cmd.status().expect("Failed to execute cargo build");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

fn run_dev(_args: &DevArgs) {
    // Start tailwind in watch mode as a background process
    let tailwind_handle = tokio::spawn(async {
        let tailwind_args = TailwindArgs {
            watch: true,
            build: false,
        };
        run_tailwind(&tailwind_args).await;
    });

    // Run the development server in the foreground
    let status = Command::new("systemfd")
        .arg("--no-pid")
        .arg("-s")
        .arg("127.0.0.1:8080")
        .arg("--")
        .arg("watchexec")
        .arg("-r")
        .arg("--stop-signal")
        .arg("SIGTERM")
        .arg("--stop-timeout=5")
        .arg("-w")
        .arg("crates")
        .arg("cargo run --package server")
        .status()
        .expect("Failed to execute start dev server. Make sure systemfd and watchexec are installed with `cargo install systemfd watchexec-cli`");

    // When the dev server exits, also stop the tailwind process
    tailwind_handle.abort();

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

async fn ensure_tailwind_binary() -> PathBuf {
    let xtask_dir = env!("CARGO_MANIFEST_DIR");
    let cache_dir = Path::new(xtask_dir).join("../../.cache");
    let version = "4.0.15";

    let binary_name = get_tailwind_binary_name();

    let tailwind_path = cache_dir
        .join(format!("tailwind@{}", version))
        .join(&binary_name);

    let url = format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/download/v{}/{}",
        version, &binary_name
    );

    download_file(&url, &tailwind_path, true)
        .await
        .expect("Failed to download Tailwind CSS binary")
}

fn get_tailwind_binary_name() -> String {
    use std::env;

    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match os {
        "macos" => match arch {
            "aarch64" => "tailwindcss-macos-arm64",
            "x86_64" => "tailwindcss-macos-x64",
            _ => panic!("Unsupported macOS architecture: {}", arch),
        },
        "linux" => {
            // Always use musl variants for Linux
            match arch {
                "aarch64" => "tailwindcss-linux-arm64-musl",
                "x86_64" => "tailwindcss-linux-x64-musl",
                _ => panic!("Unsupported Linux architecture: {}", arch),
            }
        }
        "windows" => match arch {
            "x86_64" => "tailwindcss-windows-x64.exe",
            _ => panic!("Unsupported Windows architecture: {}", arch),
        },
        _ => panic!("Unsupported OS: {}", os),
    }
    .to_string()
}

async fn run_tailwind(args: &TailwindArgs) {
    let xtask_dir = env!("CARGO_MANIFEST_DIR");

    let input_path = Path::new(xtask_dir).join("../../crates/server/assets/stylesheets/main.css");
    let output_path = Path::new(xtask_dir).join("../../crates/server/assets/build/main.css");

    // Ensure tailwind binary exists and get its path
    let tailwind_bin = ensure_tailwind_binary().await;

    let mut cmd = Command::new(tailwind_bin);

    if args.watch {
        cmd.args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--watch",
            "--cwd",
            Path::new(xtask_dir)
                .join("../../crates/server")
                .to_str()
                .unwrap(),
        ]);
    } else if args.build {
        cmd.args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--minify",
        ]);
    } else {
        eprintln!("Please specify either --watch or --build for the tailwind command");
        exit(1);
    }

    let status = cmd.status().expect("Failed to execute Tailwind CSS");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::from_env();

    match args.subcommand {
        Some(Subcommand::Build(ref build_args)) => run_build(build_args).await,
        Some(Subcommand::Dev(ref dev_args)) => run_dev(dev_args),
        Some(Subcommand::Tailwind(ref tailwind_args)) => run_tailwind(tailwind_args).await,
        None => {
            eprintln!("No subcommand provided. Use --help for available commands.");
            exit(1);
        }
    }
}
