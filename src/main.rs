use std::fs;
use std::path::PathBuf;
use std::io;
use std::env;

fn rename_and_move_files(path: &String) -> io::Result<()> {
    // Read the contents of the parent folder
    let subfolders = fs::read_dir(path)?;

    for entry in subfolders {
        let subfolder = entry?.path();
        if subfolder.is_dir() {
            // Read the contents of the subfolder
            let files = fs::read_dir(&subfolder)?;

            for file in files {
                let file = file?.path();
                let file_name = file.file_name().unwrap().to_string_lossy();
                let subfolder_name = subfolder.file_name().unwrap().to_string_lossy();
                let new_file_name = format!("{}_{}", subfolder_name, file_name);
                let new_path = PathBuf::from(path).join(new_file_name);

                // Rename and move the file
                fs::rename(&file, new_path.clone())?;
                println!("Moved and renamed: {:?} -> {:?}", file, new_path);
            }
        }
    }

    println!("All files have been renamed and moved successfully.");
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: Missing path as first argument");
        return;
    }
    let path = &args[1];
    if let Err(e) = rename_and_move_files(path) {
        eprintln!("An error occurred: {}", e);
    }
}

