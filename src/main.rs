use clap::Parser;
use platforms::Platform;
use tempfile::TempDir;
use ubi::UbiBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    project: Vec<String>,

    #[arg(short, long, help = "Keep temporary directory for build artifacts")]
    keep_temp: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // I skipped armv6 because it's not actually being built
    let targets = vec![
        Platform::find("x86_64-apple-darwin"),          // macos-x64
        Platform::find("aarch64-apple-darwin"),         // macos-arm64
        Platform::find("x86_64-unknown-linux-gnu"),     // linux-x64
        Platform::find("x86_64-unknown-linux-musl"),    // linux-x64-musl
        Platform::find("aarch64-unknown-linux-gnu"),    // linux-arm64
        Platform::find("aarch64-unknown-linux-musl"),   // linux-arm64-musl
        Platform::find("armv7-unknown-linux-gnueabi"),  // linux-armv7
        Platform::find("armv7-unknown-linux-musleabi"), // linux-armv7-musl
        Platform::find("x86_64-pc-windows-gnu"),
        Platform::find("aarch64-pc-windows-msvc"),
        Platform::find("riscv64gc-unknown-linux-gnu"),
        Platform::find("riscv64gc-unknown-linux-musl"),
    ];

    for project in cli.project.iter() {
        println!("Installing project: {}", project);

        // This will get deleted when it goes out of scope
        let temp_dir = TempDir::with_prefix("ubi-tester-")?;

        for target in targets.iter().flatten() {
            print!("Platform: {}", target.target_triple);
            let ubi = UbiBuilder::new()
                .project(project)
                .platform(target)
                .install_dir(temp_dir.path().join(target.target_triple).to_path_buf())
                .build()?;

            match ubi.install_binary().await {
                Ok(()) => println!("  Success"),
                Err(err) => println!("  Error: {}", err),
            };
        }

        // Retain the temporary directory for checking the actual files, if requested
        if cli.keep_temp {
            println!("Temp directory kept: {}", temp_dir.into_path().display());
        }
    }

    Ok(())
}
