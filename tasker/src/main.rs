use std::{
    fs::File,
    io::{self, Write},
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
        print!("Enter your todo: ");
        io::stdout().flush()?;

        let mut todo = String::new();
        io::stdin().read_line(&mut todo)?;

        let _write1 = writeln!(note, "{}", todo);
    }

    Ok(())
}
