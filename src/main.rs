use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    match content {
        Ok(content) => content,
        Err(error) => {
            return Err(error.info());
        }
    };

    println!("file conent: {}", content);
    Ok(())
}
