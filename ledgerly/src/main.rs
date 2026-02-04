use std::{
    env::{self},
    io::{Result, Write},
    path::Path,
    process::Command,
    thread::sleep,
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

fn greet(arg_in: &Vec<String>) {
    let arg_2 = arg_in.get(2);
    if arg_2.is_none() {
        println!("Hello, from Ledgerly!");
    } else {
        println!("Ledgerly greets {}", arg_in[2]);
    }
}

fn add(arg_in: &Vec<String>) -> Result<()> {
    let arg_2 = arg_in.get(2);
    if arg_2.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Hint : Input something after add argument.",
        ));
    } else {
        // return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "E"));
        create_dir_()?;
        file_handling()?;
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
    sleep_for_sec(5);
    println!("dir created succesfully");
    Command::new("mkdir").arg("./ledgerly/").output()?;
    Ok(())
}

fn file_handling() -> std::io::Result<()> {
    print!("Enter file name: ");
    std::io::stdout().flush()?;
    let mut input_name = String::new();
    std::io::stdin().read_line(&mut input_name)?;

    let final_name = format!("./ledgerly/{}.txt", input_name);

    if input_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error: unexpected blank file name",
        ));
    } else if input_name.contains(".") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error: unexpected dot (.) in file's name",
        ));
    } else if Path::new(&final_name).exists() {
        println!("The file already exists.");
        println!("Press y/n to continue/create new file");
        loop {
            print!("Enter (y/n): ");
            std::io::stdout().flush()?;

            let mut yes_or_no = String::new();
            std::io::stdin().read_line(&mut yes_or_no)?;
            let ans = yes_or_no.trim();

            if ans.is_empty() {
                println!("Error: the input was blank retry!");
                continue;
            } else if ans.len() >= 2 {
                println!(
                    "Error: expected a single char but received {} chars",
                    ans.len()
                );
                continue;
            } else if ans.contains('y') {
                let mut file = std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&final_name)?;
                let data = String::from("Hello");
                let to_bytes = data.as_bytes();
                file.write_all(&to_bytes)?;
            } else if ans.contains('n') {
                let mut file = std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&final_name)?;
                let data = String::from("Hello");
                let to_bytes = data.as_bytes();
                file.write_all(&to_bytes)?;
            }
        }
    }

    Ok(())
}

fn sleep_for_sec(n: u64) {
    let sleeping = std::time::Duration::from_secs(n);
    sleep(sleeping);
}
