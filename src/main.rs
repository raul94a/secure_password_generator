use std::{fs::OpenOptions, option};
use std::fs::File;
use std::io::Write;
use rand::{self, random, seq::SliceRandom, Rng};

fn main() {
    let minus = "abcdefghijklmnopqrstuvwxyz";
    let mayus = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "$_.,;:{}[]=)(¿?!¡<>#@$€+-*/";

    let mut charset  = String::new();
    charset.push_str(minus); 
    charset.push_str(mayus);
    charset.push_str(numbers);
    charset.push_str(minus);  
    charset.push_str(symbols);
    charset.push_str(mayus);


    let mut chars : Vec<char> = charset.chars().collect();
    // thread_rng inicializa el seed para pseudo-aleatorizar. No recomendable para criptografía, al menos
    // por defecto
    let mut rng = rand::thread_rng();
    // aleatorización de chars
    chars.shuffle(&mut rng);
    // un String es una collection de chars, por eso podemos usar el collect
    charset = chars.iter().cloned().collect();

    const LENGTH: i32 = 16;

    let password : String = (0..LENGTH).map(|_| {
        let index  = rng.gen_range(0..charset.len());
        charset.as_bytes()[index] as char // ¿Por qué? Básicamente porque solo tenemos caracteres unicodes
    }).collect();

    // Este archivo es solo para registrar las pws generadas
    // La idea es analizar qué se genera para mejorar la producción de pwds.
    let mut file = OpenOptions::new()
    .append(true)
    .create(true) // Crea el archivo si no existe
    .open("password-tracker.txt").unwrap();

    match writeln!(file,"{}",password){
        Ok(_) => println!("La contraseña se ha generado con éxito: \n{}",password),
        Err(e) => println!("Error {}", e)
    }


}
