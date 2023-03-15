use clear_file_preserve_meta as cl;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short)]
    /// set of files that need to be clearead while preserving metadata
    file_clear: Vec<String>,
    #[clap(short)]
    /// set of dirs that need to be clearead while preserving metadata
    dir_clear: Vec<String>,
    /// new content for cleared files 
    #[clap(short, long, default_value_t={"".into()})]
    new_content: String,
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    for file_path in &cli.file_clear {
        if let Err(err) = cl::change_file_content(file_path, &cli.new_content) {
            println!("cant clear file {file_path:?}: {err}")
        }
    }
    
    for dir_path in &cli.dir_clear {
        if let Err(err) = cl::change_dir_files_content(dir_path, &cli.new_content, false) {
            println!("cant clear file {dir_path:?}: {err}")
        }
    }

    Ok(())
}
