use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let mut avg_tries = Vec::new();
    for i in 0..100 {
        let tries = binary_search();
        avg_tries.push(tries);
    }
    println!("👉 Average Tries : {}", avg_tries.iter().sum::<i32>() / avg_tries.len() as i32);
}

fn random_search() -> i32 {
    let mut previous_max: i32 = 101;
    let mut previous_min: i32 = 1;
    let mut count_tries: i32 = 0;

    let secret = rand::thread_rng().gen_range(1..101);
    println!(" ********* Secret Number : {secret} ********** ");
    println!("👉 Guess the number !");

    loop {
        count_tries += 1;
        let guess_number = get_random_number(previous_min, previous_max);
        println!("ℹ️ Your number is : {guess_number}");

        match guess_number.cmp(&secret) {
            Ordering::Less => {
                println!("Try Up 👆");
                previous_min = guess_number;
            }
            Ordering::Greater => {
                println!("Try Down 👇");
                previous_max = guess_number;
            }
            Ordering::Equal => {
                println!("Bravo ! 🎉 ");
                println!("👉 Secret Number : {secret}");
                println!("👉 Your Number : {guess_number}");
                println!("👉 Previous Max : {previous_max}");
                println!("👉 Previous Min : {previous_min}");
                println!("👉 Count Tries : {count_tries}");
                break;
            }
        }
    }
    return count_tries;
}

fn get_random_number(min: i32, max: i32) -> i32 {
    return rand::thread_rng().gen_range(min..max);
}

fn binary_search() -> i32 {

    let mut previous_max: i32 = 101;
    let mut previous_min: i32 = 1;
    let mut count_tries: i32 = 0;

    let secret = rand::thread_rng().gen_range(1..101);
    println!(" ********* Secret Number : {secret} ********** ");
    println!("👉 Guess the number !");
    loop {
        count_tries += 1;
        let guess_number = find_middle(previous_min, previous_max);
        println!("ℹ️ Your number is : {guess_number}");

        match guess_number.cmp(&secret) {
            Ordering::Less => {
                println!("Try Up 👆");
                previous_min = guess_number;
            }
            Ordering::Greater => {
                println!("Try Down 👇");
                previous_max = guess_number;
            }
            Ordering::Equal => {
                println!("Bravo ! 🎉 ");
                println!("👉 Secret Number : {secret}");
                println!("👉 Your Number : {guess_number}");
                println!("👉 Previous Max : {previous_max}");
                println!("👉 Previous Min : {previous_min}");
                println!("👉 Count Tries : {count_tries}");
                break;
            }
        }
    }
    return count_tries;
}

fn find_middle(p0: i32, p1: i32) -> i32 {
       return (p0 + p1) / 2;
}
