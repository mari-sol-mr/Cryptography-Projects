use std::io;
use indexmap::IndexMap;

fn calculate_frequency(base: u64, modulus: u64, revolutions: u64) {
    let mut freq: IndexMap<u64, u32> = IndexMap::new();

    for exp in 0..(revolutions*modulus - (revolutions)) {
        let remainder = mod_pow_recursive(base, exp, modulus);
        *freq.entry(remainder).or_insert(0) += 1;
    }


    for (remainder, f) in &freq {
        let formatted_x = format_x(remainder);
        println!("\t{} => {}", formatted_x, f);
    }

    if freq.len() as u64 == modulus - 1 {
        println!("\n\n
        The base you entered is a primitive root of {}. 
        This means that you can get all the possible remainders of mod {}.", modulus, modulus)
    }
    else {
        println!("\n\n
        The base you entered is not a primitive root of {}. 
        This means that you cannot get all the possible remainders of mod {}.
        The only remainders you can get are: \t\n", modulus, modulus);
        
        let keys: Vec<u64> = freq.keys().cloned().collect();
        print!("\t{}", keys[0]);
        for i in 1..keys.len() {
            print!(", {}", keys[i]);
        }
    }
    println!("\n");
}

fn mod_pow_recursive(base: u64, exp: u64, modulus: u64) -> u64 {
    if exp == 0 {
        return 1;
    }
    let half = mod_pow_recursive(base, exp / 2, modulus);
    let mut result = (half * half) % modulus;
    if exp % 2 == 1 {
        result = (result * base) % modulus;
    }
    result
}

fn format_x(x: &u64) -> String {
    const WIDTH: usize = 5;
    let x_str = x.to_string();
    if x_str.len() > WIDTH {
        format!("{:.3}...", x_str)
    } else {
        format!("{:<6}", x_str)
    }
}

fn main() {
    println!("\n
    \tThis program calculates the 'residue class' (a.k.a'the ring') of a^x mod n, where
    \ta and n are constant. It shows all the possible remainders and their distribution
    \tfor some number of revolutions. It will tell you if the base you chose is the 
    \tprimitive root of mod n, meaning that it is able to produce all the possible 
    \tremainders of mod n (the residue class modulo n). 

    \tFor example: 3^x mod 5 calculated for 2 revolutions would print out:

    \t1 => 2
    \t3 => 2
    \t4 => 2
    \t2 => 2



    \tIt uses an efficient way of calcaulting the modulus of big numbers (function big_number).
    ");

    let mut input = String::new();

    println!("\tEnter the modulus: ");
    io::stdin().read_line(&mut input).expect("\tFailed to read");

    let modulus: u64 = input.trim().parse().expect("\tPlease type a number!");

    input.clear();

    println!("\tEnter the base: ");
    io::stdin().read_line(&mut input).expect("\tFailed to read");

    let base: u64 = input.trim().parse().expect("\tPlease type a number!");

    println!("\tYou entered modulus: {} and base: {}", modulus, base);

    input.clear();

    println!("\tEnter number of revolutions to calculate: ");
    io::stdin().read_line(&mut input).expect("\tFailed to read");

    let revolutions: u64 = input.trim().parse().expect("\tPlease type a number!");

    println!("\n\n\tYou entered modulus: {} and base: {}.
    \n\tNumber of revolutions to calculate: {}.", modulus, base, revolutions);

    calculate_frequency(base, modulus, revolutions);
}
