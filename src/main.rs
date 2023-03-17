use clear_file_preserve_meta as cl;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short)]
    /// set of files that need to be clearead while preserving metadata
    file_clear: Vec<String>,
    #[clap(short, verbatim_doc_comment)]
    /// set of dirs that need to be clearead while preserving metadata
    /// by default clearing is non-recursive
    /// for recursive dir cllearing add to beginning `+`
    /// for explicit non-recursive dir cllearing add to beginning `!` 
    dir_clear: Vec<String>,
    /// new content for cleared files 
    #[clap(short, long, default_value_t={"\n".into()})]
    new_content: String,
    /// white list regex for cleared files
    #[clap(short='w',long="wlr")]
    white_list_regex: Option<String>,
    /// regex black list for cleared files
    #[clap(short='b',long="blr")]
    black_list_regex: Option<String>,
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    let white_list = cli.white_list_regex.as_ref().map(|re|re.as_str());
    let black_list = cli.black_list_regex.as_ref().map(|re|re.as_str());

    let file_filter = cl::FileFilter::new(white_list, black_list).unwrap_or_else(|err|{
        panic!("regex error: {err}")
    });
    let file_filter = &file_filter;


    for file_path in &cli.file_clear {
        if let Err(err) = cl::change_file_cont_filter_f(file_path, &cli.new_content, file_filter) {
            println!("cant clear file {file_path:?}: {err}")
        }
    }
    
    for dir_path in &cli.dir_clear {
        // non_recursive for files with first char '+'
        let non_recursive = dir_path.starts_with("!");
        let recursive = !non_recursive && dir_path.starts_with("+");
        let dir_path = if recursive || non_recursive { &dir_path[1..] } else { dir_path };

        if let Err(err) = cl::change_dir_files_cont_filter_f(dir_path, &cli.new_content, recursive, file_filter) {
            println!("cant (completely) clear dir {dir_path:?}: {err}")
        }
    }

    Ok(())
}
