use clear_file_preserve_meta as cl;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short)]
    /// set of files that need to be clearead while preserving metadata
    clear: Vec<String>,
    /// new content for cleared files 
    #[clap(short, long, default_value_t={"".into()})]
    new_content: String,
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    for file_path in &cli.clear {
        if let Err(err) = cl::change_file_content(file_path, &cli.new_content) {
            println!("cant clear file {file_path:?}: {err}")
        }
    }
    
    Ok(())
}
