use std::fs;
use std::path::Path;

pub fn read_directory(
    path: &Path,
    file_type: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut read_dir: Vec<String> = Vec::new();

    let path_str = path.to_str().unwrap_or_default();

    if file_type.is_empty() {
        read_dir = fs::read_dir(path_str)?
            .filter_map(|e| e.ok())
            .filter_map(|e| e.path().to_str().map(|s| s.to_string()))
            .collect();
    }

    // read_dir.into_iter().for_each(|e| println!("{e}"));
    Ok(read_dir)
}

pub fn read_directory_recursive(
    path: &Path,
    file_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    read_directory(path, file_type)?;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            read_directory_recursive(&entry_path, file_type)?;
        }
    }

    Ok(())
}
