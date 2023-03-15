use std::io::Write;
use std::path::Path;
use filetime::FileTime;

fn clear_file(path: impl AsRef<Path>, clear_str: &str) -> std::io::Result<()> {
    let path = path.as_ref();

    let md = std::fs::metadata(path)?;
    // let ctime = FileTime::from_system_time(md.created()?);
    let mtime = FileTime::from_system_time(md.modified()?);
    let atime = FileTime::from_system_time(md.accessed()?);

    {
        let mut f = std::fs::File::create(path)?;
        if !clear_str.is_empty() {
            write!(f, "{clear_str}")?;
        }
    }

    filetime::set_file_times(path, atime, mtime)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    Ok(())
}


#[cfg(test)]
#[test]
fn test_file_clear() -> std::io::Result<()> {
    use std::io::Read;

    let dir = "./tests";
    let path = format!("{dir}/test_file_clear.txt");
    let path = path.as_str();

    // init file:
    {
        std::fs::create_dir_all(dir)?;
        let mut f = std::fs::File::create(path)?;
        writeln!(f, "initial content\n\nrand_str = hvQZHipJXYDTnFVG\n...\nEND")?;
    }

    // save file md:
    let md = std::fs::metadata(path)?;
    let mtime_init = md.modified()?;

    // slightly await:
    std::thread::sleep(std::time::Duration::from_secs(3));

    // clear file preserve md:
    let clear_str = "#[error]";
    clear_file(path, clear_str)?;

    // slightly await:
    std::thread::sleep(std::time::Duration::from_secs(2));

    // take cleared file md:
    let md = std::fs::metadata(path)?;
    let mtime = md.modified()?;

    // assert that md is preserved:
    assert_eq!(mtime_init, mtime);

    // take cleared file content:
    let mut new_file_content = String::new();
    std::fs::File::open(path)?.read_to_string(&mut new_file_content)?;
    
    // assert that file content changed:
    assert_eq!(new_file_content, clear_str);

    Ok(())
}
