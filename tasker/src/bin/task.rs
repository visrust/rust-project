use std::{
    env,
    fs::File,
    io::{self, Write}, path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    file_creation(&mut input)?;
    Ok(())
}

fn file_creation(input :  &mut String) -> Result<(), Box<dyn std::error::Error>>{
    loop {
        print!("Enter File name: ");
        io::stdout().flush()?;
        io::stdin().read_line(input)?;
        let trimmed_input = input.trim();
        let filename = format!("{}.txt", input.trim());

        if trimmed_input.contains(".") {
            println!("Error[1]: File name conatins extesion");
            continue;
        }
        if trimmed_input.is_empty() {
            println!("Error[2]: File name can not be empty");
            continue;
        }
        let path = Path::new(&filename);
        if path.exists() {
            println!("Error[3]: The file already exists");
            continue;
        } 
        let _ = File::create_new(&filename);

        let mut file_path = env::current_dir()?;
        file_path.push(&filename);

        println!("File {filename} saved in {}", file_path.display());
        break;
    }
    Ok(())

}
