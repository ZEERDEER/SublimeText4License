use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn replace_bytes(file_path: &str, old_bytes: &[u8], new_bytes: &[u8]) -> io::Result<()> {
    // Check if file exists
    if !PathBuf::from(file_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error: File '{}' does not exist", file_path),
        ));
    }

    // Check if byte sequences have the same length
    if old_bytes.len() != new_bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Error: Byte sequences must have the same length",
        ));
    }

    // Read file content
    let data = fs::read(file_path)?;

    // Check if target byte sequence exists
    if !data.windows(old_bytes.len()).any(|window| window == old_bytes) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error: Target byte sequence not found in file",
        ));
    }

    // Replace byte sequence
    let new_data: Vec<u8> = data
        .windows(old_bytes.len())
        .enumerate()
        .flat_map(|(i, window)| {
            if i > 0 && data.windows(old_bytes.len()).nth(i - 1).unwrap() == old_bytes {
                return vec![];
            }
            if window == old_bytes {
                new_bytes.to_vec()
            } else {
                vec![data[i]]
            }
        })
        .collect();

    // Create backup file
    let backup_path = format!("{}.bak", file_path);
    fs::write(&backup_path, &data)?;
    println!("Backup created: {}", backup_path);

    // Write modified file
    fs::write(file_path, new_data)?;
    println!("File successfully modified: {}", file_path);

    Ok(())
}

fn main() {
    // Get current executable path
    let exe_path = std::env::current_exe().expect("Unable to get program path");
    let exe_dir = exe_path.parent().expect("Unable to get program directory");
    let target_path = exe_dir.join("sublime_text.exe");

    let old_bytes = [0x80, 0x79, 0x05, 0x00, 0x0F, 0x94, 0xC2];
    let new_bytes = [0xC6, 0x41, 0x05, 0x01, 0xB2, 0x00, 0x90];

    if let Err(e) = replace_bytes(
        target_path.to_str().unwrap(),
        &old_bytes,
        &new_bytes,
    ) {
        eprintln!("{}", e);
    } else {
        println!("\nOperation completed!");
    }
    
    print!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}