use std::{fs::File, io::{self, Write}};

fn _main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    loop {
        print!("Enter File name: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let trimmed_input = format!("{}.txt", input.trim());

        if trimmed_input.contains(".") | trimmed_input.is_empty(){
            println!("Error[1]: Either conatins extesion or is empty");
        } else {
            let extensioned_file = File::create_new(trimmed_input);
            println!("File {input} saved at ./{extensioned_file:?}");
            break;
        }
    }
    Ok(())
}
