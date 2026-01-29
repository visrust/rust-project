use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut buf_input = file_creation(&mut input)?;
    let _ = editor(&mut buf_input);
    Ok(())
}

fn file_creation(input: &mut String) -> Result<File, Box<dyn std::error::Error>> {
    let file = loop {
        // Clearning the input
        input.clear();
        print!("Enter File name: ");
        io::stdout().flush()?;
        io::stdin().read_line(input)?;
        let trimmed_input = input.trim();
        let filename = format!("{}.txt", input.trim());

        // basic logic
        if trimmed_input.contains(".") {
            println!("Error[1]: File name conatins extesion");
            // continue keeps program running
            continue;
        }
        if trimmed_input.is_empty() {
            println!("Error[2]: File name can not be empty");
            continue;
        }

        // currently needed because of mercy handling
        let path = Path::new(&filename);
        if path.exists() {
            println!("Error[3]: The file already exists");
            continue;
        }
        let note_file = File::create_new(&filename)?;

        let mut file_path = env::current_dir()?;
        file_path.push(&filename);

        println!("File {filename} saved in {}", file_path.display());
        break note_file;
    };
    Ok(file)
}

fn editor(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    println!("Editing into the file");
    loop {
        let mut writer = String::new();
        let byte_written = io::stdin().read_line(&mut writer)?;
        // Interception on Ctrl-D
        if byte_written == 0 {
            writer.push('\n');
            break;
        }
        file.write_all(&writer.as_bytes())?;
        // let trim_writer = writer.trim();
    }
    Ok(())
}
