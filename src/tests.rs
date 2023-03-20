use std::io::{Read, Write};
use crate::ClearFile;

#[test]
fn test_file_clear() -> std::io::Result<()> {

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
    let new_content = "#[error]";
    crate::clear_act::ConstChangeContF::new_no_filter(new_content).clear_file(path)?;

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
    assert_eq!(new_file_content, new_content);

    Ok(())
}
