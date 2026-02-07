use std::{
    env::{self},
    io::{Result, Write},
};

// prototype code
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // if argument is less than 2 then print help
    if args.len() < 2 {
        help();
        return Ok(());
    }

    // argument 2 is always sub command
    let sub_command = &args[1];

    // used as_string as we can't match directly on String/&String
    match sub_command.as_str() {
        "greet" => greet(&args),
        "add" => add(&args)?,
        "help" => help(),
        _ => println!("Wrong command! Use this instead : ledgerly help"),
    }
    Ok(())
}

fn greet(arg_in: &[String]) {
    let arg_2 = arg_in.get(2);
    if arg_2.is_none() {
        println!("Hello, from Ledgerly!");
    } else {
        println!("Ledgerly greets {}", arg_in[2]);
    }
}

fn add(arg_in: &[String]) -> Result<()> {
    let arg_2 = arg_in.get(2);
    if arg_2.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Hint : Input something after add argument.",
        ));
    } else {
        let to_add = arg_in[2..].join(" ");
        let prompt: &str = &to_add;
        create_dir_()?;
        let path = file_name_check()?;
        let path_str: &str = &path;

        if std::path::Path::new(&path).exists() {
            file_exists_in_path(path_str, prompt)?;
        } else {
            std::fs::write(&path, prompt)?;
        }
    }

    Ok(())
}

fn help() {
    println!(
        r#"
        Usage : ledgerly <cmd>
        Where cmd is :
        
        1. add               --> ro add a value
        2. sum               --> for summarize.
        3. help              --> for help.
        4. greet             --> for greeting.
        5. since DD-MM-YY    --> to see from a time.
        6. today             --> to see today's ledger.
        7. yesterday         --> to see yesterday ledger.
        8. week              --> to see past 7 day ledger.

    "#
    )
}

fn create_dir_() -> Result<()> {
    println!("creating dir...");
    std::fs::create_dir_all("./ledgerly/")?;
    Ok(())
}

fn get_input(prompt: &str) -> std::io::Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn file_name_check() -> std::io::Result<String> {
    let input_name = get_input("Enter file's name: ")?;
    let path = format!("./ledgerly/{}.txt", input_name);

    if input_name.is_empty() {
        println!("Input name can not be empty! Retry...");
    }

    if input_name.contains(".") {
        println!("Not allowed to add extension or dots in file name")
    }

    if std::path::Path::new(&path).exists() {
        println!("Path already exists!");
    }
    Ok(path)
}

fn file_exists_in_path(path: &str, prompt: &str) -> std::io::Result<()> {
    loop {
        let ans = get_input("File exists! Overwrite? (y/n): ")?;
        match ans.as_str() {
            "y" => {
                std::fs::write(path, prompt)?;
                break;
            }

            "n" => {
                break;
            }

            _ => {
                println!("Please enter `y` or `n`");
                continue;
            }
        }
    }
    Ok(())
}
