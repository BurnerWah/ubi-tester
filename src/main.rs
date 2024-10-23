use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    project: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("projects: {:?}", cli.project);
}
