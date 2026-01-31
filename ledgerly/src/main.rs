use std::{env, error::Error};

// prototype code
type TryOp<T> = Result<T, Box<dyn Error>>;

fn main() -> TryOp<()> {
    let args: Vec<String> = env::args().collect();
    // Basic logic
    if args.len() < 2 {
        println!("Usage : ledgerly <cmd>");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "greet" => greet(&args),
        "add" => add(&args),
        "help" => help(),

        // "add" => add_cmd()?,
        _ => println!("For usage help : ledgerly help"),
    }

    // if command.contains("add") {
    //     if args.len() < 3 {}
    // }
    Ok(())
}

// fn add_cmd() -> io::Result<()> {
//     let arg_add: Vec<String> = env::args().collect();
//     if arg_add.len() < 3 {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidInput,
//             "Error : Atleast 3 parameters needed to proceed. Help : try using --> ledgerly add water intake at 12 pm was 120ml",
//         ));
//     } else {
//         println!("Added successfully!")
//     }
//     Ok(())
// }

fn greet(arg_in: &Vec<String>) {
    println!("Hello! {}", arg_in[2]);
}

fn add(arg_in: &Vec<String>) {
    println!(
        "Added : {}, {}, {}, {}",
        arg_in[2], arg_in[3], arg_in[4], arg_in[6]
    );
}

fn help() {
    println!(
        r#"
        Usage : ledgerly <cmd>
        Note : // is used as comments
        Where cmd is :
        
        1. add // to add a value
        2. sum // for summarize.
        3. help // for help.
        4. greet // for greeting.
        5. since DD/MM/YYYY // to see from a time.
        6. today // to see today's ledger.
        7. yesterday // to see yesterday ledger.
        8. week // to see past 7 day ledger.

    "#
    )
}
