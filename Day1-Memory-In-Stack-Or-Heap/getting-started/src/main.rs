/*
 * @Date: 2021-09-26 00:41:51
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-09-26 01:59:47
 */

use rand::{Rng, random, thread_rng};
use std::io::{self, Read};


fn main() {
    let literal = "Hello, world!";
    let s = literal.as_bytes();
    let str = literal.to_string();
    println!("{}", literal);
    println!("{:?}, {:p}, {:p}, {:p}, {:p}, {:p} 😝", s, s, &s, literal, &literal, &str);
    loop {
        let mut guess = String::new();
        // let token:i32 = random();
        let num = thread_rng().gen_range(1..100);
        io::stdin().read_line(&mut guess).unwrap();
        println!("{:?}", guess.as_bytes());
        let guess:i32 = guess.strip_suffix("\n").unwrap().parse().unwrap();
        println!("You guessed: {}", guess);
        println!("The secret number is: {}", num);
        println!("{} vs. {} is {}", guess, num, guess == num);
        if guess == num {
            break;
        }
    }
}
