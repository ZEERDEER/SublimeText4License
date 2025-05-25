use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;

// Function to find and replace hex byte sequences
fn find_and_replace(data: &mut Vec<u8>, pattern: &[u8], replacement: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;

    while i <= data.len() - pattern.len() {
        if data[i..i+pattern.len()] == *pattern {
            // Replace matching bytes
            for j in 0..replacement.len() {
                data[i+j] = replacement[j];
            }
            count += 1;
            i += pattern.len();
        } else {
            i += 1;
        }
    }

    count
}

fn main() -> io::Result<()> {
    // Get current directory
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join("sublime_text.exe");

    // Check if file exists
    if !file_path.exists() {
        eprintln!("Error: 'sublime_text.exe' not found in the current directory.");
        return Ok(());
    }

    // Read file data
    let mut file_data = fs::read(&file_path)?;
    println!("Loaded file: {:?} ({} bytes)", file_path.file_name().unwrap(), file_data.len());

    // Rule 1 - Find 74 06 3B, change 74 to EB
    let pattern1 = [0x74, 0x06, 0x3B];
    let replacement1 = [0xEB, 0x06, 0x3B];
    let count1 = find_and_replace(&mut file_data, &pattern1, &replacement1);
    println!("Rule 1: Changed '74 06 3B' to 'EB 06 3B' - {} occurrence(s) modified", count1);

    // Rule 2 - Find 89 F8 48 81 C4 38 02, change first two bytes to 33 C0
    let pattern2 = [0x89, 0xF8, 0x48, 0x81, 0xC4, 0x38, 0x02];
    let replacement2 = [0x33, 0xC0, 0x48, 0x81, 0xC4, 0x38, 0x02];
    let count2 = find_and_replace(&mut file_data, &pattern2, &replacement2);
    println!("Rule 2: Changed '89 F8 48 81 C4 38 02' to '33 C0 48 81 C4 38 02' - {} occurrence(s) modified", count2);

    // If any changes, save file
    if count1 > 0 || count2 > 0 {
        // Create backup
        let backup_path = file_path.with_extension("exe.bak");
        fs::copy(&file_path, &backup_path)?;
        println!("Backup created: {:?}", backup_path.file_name().unwrap());

        // Write modified data
        fs::write(&file_path, &file_data)?;
        println!("Patched file saved: {:?}", file_path.file_name().unwrap());
        println!("Total modifications: {}", count1 + count2);
    } else {
        println!("No matching byte sequences found. File not modified.");
    }

    Ok(())
} 