use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct ApiConfig {
    client_name: String,
    client_id: String,
    client_secret: String, //Endpoints...
                           //Queries...
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let pb = indicatif::ProgressBar::new(100);

    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    let config = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let cfg: ApiConfig;
    if (args.length() > 1) {
        cfg = confy::load("rurl", config);
    } else {
        cfg = confy::load("rurl", None)?;
    }
    Ok(())
}
