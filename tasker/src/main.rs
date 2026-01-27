use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to tasker");

    print!("Enter name of Task file : ");
    io::stdout().flush()?;

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;

    let file = file_name.trim();

    if file.is_empty() {
        println!("File name can't be empty");
    } else if file.contains(".") {
        println!("Adding .extension is not allowed")
    } else {
        let full_path = format!("{}.txt", file);
        let mut note = File::create_new(&full_path)?;

        loop {
            print!("Enter your todo: ");
            io::stdout().flush()?;

            let mut todo = String::new();
            io::stdin().read_line(&mut todo)?;

            if todo.trim() == ":q" {
                println!("file saved at ./{}", full_path);
                let status = Command::new("bat").arg(format!("./{file}.txt")).status()?;

                if !status.success() {
                    println!("Command Failed to run! Install bat\n Continuing with cat");
                    let status = Command::new("cat").arg(format!("./{file}.txt")).status()?;
                    if !status.success() {
                        println!("Failed");
                    }
                }
                break;
            } else if todo.trim() == ":w" {
                println!("saved!");
            } else if todo.trim() == ":h" {
                println!(":q or :w ==> for quit or write\n empty line ==> treated as enter");
            } else if todo.trim().is_empty() {
                println!("Will be treated as line break");
            }
            // write
            note.write_all(todo.as_bytes())?;
        }
    }

    Ok(())
}
