use crate::arrays;
use crate::conditionals;
use crate::enums;
use crate::functions;
use crate::loops;
use crate::pointer_ref;
use crate::print;
use crate::strings;
use crate::structs;
use crate::tuples;
use crate::types;
use crate::vars;
use crate::vectors;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command.");
        println!("\nAvailable commands:");
        println!("  print, vars, types, strings, tuples, arrays,");
        println!("  vectors, conditionals, functions, loops, structs, enums, pointer_ref");
        return;
    }

    match args[1].as_str() {
        "print" => print::run(),
        "vars" => vars::run(),
        "types" => types::run(),
        "strings" => strings::run(),
        "tuples" => tuples::run(),
        "arrays" => arrays::run(),
        "vectors" => vectors::run(), // Added missing comma here
        "conditionals" => conditionals::run(),
        "functions" => functions::run(),
        "loops" => loops::run(),
        "structs" => structs::run(),
        "enums" => enums::run(),
        "pointer_ref" => pointer_ref::run(),
        "help" | "--help" | "-h" => {
            println!("Rust Learning Project - CLI");
            println!("\nUsage: cargo run <command>");
            println!("\nAvailable commands:");
            println!("  print         - Print examples");
            println!("  vars          - Variable demonstrations");
            println!("  types         - Data type examples");
            println!("  strings       - String operations");
            println!("  tuples        - Tuple examples");
            println!("  arrays        - Array demonstrations");
            println!("  vectors       - Vector operations");
            println!("  conditionals  - Conditional statements");
            println!("  functions     - Function examples");
            println!("  loops         - Loop demonstrations");
            println!("  structs       - Struct examples");
            println!("  enums         - Enum demonstrations");
            println!("  pointer_ref   - Pointer and reference examples");
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Run 'cargo run help' to see available commands.");
        }
    }
}
