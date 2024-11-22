import os

def replace_bytes(file_path, old_bytes, new_bytes):
    try:
        # Check if path exists
        if not os.path.exists(file_path):
            print(f"Error: File '{file_path}' does not exist.")
            return

        # Read file content
        with open(file_path, 'rb') as f:
            data = f.read()
        
        # Verify target byte sequence exists
        if old_bytes not in data:
            print("Error: Target byte sequence not found in file.")
            return

        # Replace byte sequence
        modified_data = data.replace(old_bytes, new_bytes)

        # Create backup file
        backup_path = file_path + '.bak'
        with open(backup_path, 'wb') as f:
            f.write(data)
        print(f"Backup created: {backup_path}")

        # Write modified file
        with open(file_path, 'wb') as f:
            f.write(modified_data)
        print(f"File successfully modified: {file_path}")

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    # Get file path from user input
    file_path = input("Please enter file path: ").strip()
    # Byte sequences to replace
    old_bytes = b'\x80\x79\x05\x00\x0F\x94\xC2'
    new_bytes = b'\xC6\x41\x05\x01\xB2\x00\x90'

    # Call function to perform replacement
    replace_bytes(file_path, old_bytes, new_bytes)
