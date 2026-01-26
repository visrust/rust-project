use std::{fs::File, io::{self, Write}};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Welcome to tasker");

    print!("Enter name of Task file : ");
    io::stdout().flush()?;

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;


    let space_char = " ".as_bytes();

    if file_name.as_bytes() == space_char {
        println!("Name can't have space")
    } else if file_name.contains(".") {
        println!("Adding .extension is not allowed")
    } else {
        let mut note = File::create_new(file_name)?;
        print!("Enter your todo: ");
        io::stdout().flush()?;

        let mut todo = String::new();
        io::stdin().read_line(&mut todo)?;
        
        let _write1 = writeln!(note, "{}",todo);
    }

    Ok(())
}
