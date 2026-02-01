use std::{env, fs::File, io::Result, path::{ Path}};

// prototype code
type TryOp<T> = Result<T>;

fn main() -> TryOp<()> {
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
        "add" => add(&args),
        "help" => help(),
        _ => println!("Wrong command! Use this instead : ledgerly help"),
    }
    Ok(())
}

fn greet(arg_in: &Vec<String>) {
    println!("Hello! {}", arg_in[2]);
}

fn add(arg_in: &Vec<String>) {
    if arg_in.len() <= 2 {
        println!("Nothing has been added after the add argument!");
    }

    let item = &arg_in[2..];

    if arg_in.len() >= 3 {
        println!("Added : {}", item.join(","));
        let x = _file_handling();
        return x;
        // shift this logic in a function
    }
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

fn create_dir_() -> Result<(), std::error::Error>{
    let dir = std::fs::create_dir("./ledgerly")?;
    return dir;

}

fn _file_handling()-> std::io::Result<()>{
    let file_path = "./ledgerly/today.txt";
    let _file = File::create_new(file_path);
    let path = Path::new(file_path);
    println!("File path : {:?}", path);
    // let new_file = File::create("./ledgerly/today.txt")?;
    // let path = Path::new(&new_file);
    Ok(())
}

