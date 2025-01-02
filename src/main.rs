use clap::Parser;
use rand::{self, seq::SliceRandom, Rng};
use std::fs::OpenOptions;
use std::io::Write;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of characters in the password
    #[arg(short, long, default_value = "12")]
    length: i32,

    /// Verbose mode
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// Permite repeticiones de caracteres
    #[arg(short, long, default_value = "true")]
    allow_repetitions: bool,

    /// Maximum number of special characters
    #[arg(short, long, default_value = "3")]
    symbols: i32,

    /// Maximum number of lower characters
    #[arg(short, long, default_value = "12")]
    lower: i32,

    /// Maximum number of upper characters
    #[arg(short, long, default_value = "12")]
    upper: i32,
}
const MINUS: &str = "abcdefghijklmnopqrstuvwxyz";
const MAYUS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "$_.,;:{}[]=)(¿?!¡<>#@$€+-*/";
fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Numero de simbolos permitidos {}", args.symbols);
    }

    let verbose: bool = args.verbose;

    let mut charset = String::new();
    charset.push_str(MINUS);
    charset.push_str(NUMBERS);
    charset.push_str(SYMBOLS);
    charset.push_str(MAYUS);

    let mut chars: Vec<char> = charset.chars().collect();
    // thread_rng inicializa el seed para pseudo-aleatorizar. No recomendable para criptografía, al menos
    // por defecto
    let mut rng = rand::thread_rng();
    // aleatorización de chars
    chars.shuffle(&mut rng);

    // let mut vec_char = Vec::<char>::new();

    let mut password = String::new();

    for _ in 0..args.length {
        let char = get_random_char_with_limit(&chars, &password, &args);
        password += &char.to_string();
    }

    if verbose {}
    // Este archivo es solo para registrar las pws generadas
    // La idea es analizar qué se genera para mejorar la producción de pwds.

    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // Crea el archivo si no existe
        .open("password-tracker.txt")
        .unwrap();

    match writeln!(file, "{}", password) {
        Ok(_) => {
            println!("{}", password);
        }
        Err(e) => println!("Error {}", e),
    }
}

fn get_random_char_with_limit(vec: &Vec<char>, growing_str: &String, args: &Args) -> char {
    let mut rng = rand::thread_rng();
    let mut char: char = '\0';
    let index = rng.gen_range(0..vec.len());
    char = vec[index];
    // caso A -> Max numero de symbols
    let nr_symbols = get_nr_symbols_password(growing_str);
    if args.verbose {
        println!("Numero de simbolos de la contraseña: {}", nr_symbols);
    }
    if SYMBOLS.contains(char) && nr_symbols >= args.symbols {
        if args.verbose {
            println!("Obteniendo nuevo valor");
        }
        while SYMBOLS.contains(char) {
            let index = rng.gen_range(0..vec.len());
            char = vec[index];
            if args.verbose {
                println!("Char: {}", char);
            }
        }
    }
    // CASO B -> no se permiten las repeticiones de caracteres
    
    if(!args.allow_repetitions){
        while growing_str.contains(char){
            let index = rng.gen_range(0..vec.len());
            char = vec[index];
            
        }
    }
    return char;
}

fn get_nr_symbols_password(growing_str: &String) -> i32 {
    return growing_str.chars().into_iter().fold(0, |acc, e| {
        if SYMBOLS.contains(e) {
            return acc + 1;
        }
        return acc;
    });
}
