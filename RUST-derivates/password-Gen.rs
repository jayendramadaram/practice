

use std::io::{ BufRead};

use rand::{Rng, seq::SliceRandom};

/**
 * Rules for a strong pw [Only supports length of 20]
 * A .. Z - 1
 * a .. z - 1
 * Special Chars - 1
 * 1 .. 9 - 1
 * randomly choose rest of the characters
 * 
 * ASCII for passwords
 * ok special chars are scatterred to pick rather we maintain a list and pick one 
 * 48..=57 -> 0 - 9 chars
 * 65..=90 -> A - Z 
 * 97..=122 -> a - z 
 * 33..126 -> rest of the chars
 */
fn main() {
    let mut line = String::new();
    
    let mut randomizer = rand::thread_rng();
    
    println!("Enter Length of password : ");
    std::io::stdin().lock().read_line(&mut line).unwrap();
    let length= 20.min(line.trim().parse::<u32>().unwrap());

    let mut pass_word = Vec::new();
    
    // choose special character
    const CHAR_ARRAY : [char ; 32] = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
    pass_word.push(CHAR_ARRAY[randomizer.gen_range(0..=32)]);
    pass_word.push(char::from_u32( randomizer.gen_range(48..=57)).unwrap());
    pass_word.push(char::from_u32( randomizer.gen_range(65..=90)).unwrap());
    pass_word.push(char::from_u32( randomizer.gen_range(97..=122)).unwrap());
    (0..(length - 4)).for_each(|_| {
        pass_word.push(char::from_u32(randomizer.gen_range(33..=126)).unwrap());
    });
    pass_word.shuffle(&mut randomizer);
    let final_password : String = pass_word.into_iter().collect();
    println!("Here is your Final Strong Password{:?}" , final_password)
    
} 