import os
import shutil

def patch_sublime_text(exe_path):
    # Original and replacement byte sequences
    original_bytes = bytes([0x80, 0x79, 0x05, 0x00, 0x0F, 0x94, 0xC2])
    replacement_bytes = bytes([0xC6, 0x41, 0x05, 0x01, 0xB2, 0x00, 0x90])
    
    # Check if file exists
    if not os.path.exists(exe_path):
        print(f"Error: File not found {exe_path}")
        return
    
    # Create backup file
    backup_path = exe_path + '.backup'
    shutil.copy2(exe_path, backup_path)
    print(f"Backup file created: {backup_path}")
    
    # Read file content
    with open(exe_path, 'rb') as f:
        content = f.read()
    
    # Find and replace byte sequence
    if original_bytes in content:
        new_content = content.replace(original_bytes, replacement_bytes)
        
        # Write modified content
        with open(exe_path, 'wb') as f:
            f.write(new_content)
        print("Patch successfully applied!")
    else:
        print("Target byte sequence not found, file unchanged.")

def main():
    sublime_path = input("Please enter the full path to Sublime Text: ")
    sublime_path = sublime_path.strip('"')
    
    try:
        patch_sublime_text(sublime_path)
    except PermissionError:
        print("Error: Insufficient permissions to modify file. Please run this script as administrator.")
    except Exception as e:
        print(f"An error occurred: {str(e)}")

if __name__ == "__main__":
    main()