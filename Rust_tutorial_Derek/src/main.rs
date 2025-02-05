#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Basics

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you!";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't receive Input!");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

//Data types

// fn main(){
//     const ONE_MIL:i32 = 1_000_000;
//     let age = "30";
//     let mut age:u32 = age.trim().parse()
//         .expect("Age wasn't a number !");
//     age = age+1;
//     println!("I'm {}, and I want ${}", age, ONE_MIL);
// }

//Math

// fn main() {
//     let random_num = rand::rng().random_range(1..101);
//     println!("Random Number is: {}", random_num);
// }

//If

// fn main() {
//     // let age:i32 = io::stdin().read_line();
//
//     let age = 30;
//     if(age==18){
//         println!("Important Birthday!");
//     }
//     else if(age==21){
//         println!("Important Birthday!");
//     }
//     else{
//         println!("Not an important birthday!");
//     }
// }

//Match

// fn main() {
//     let age = 20;
//     match age {
//         1..=18 => println!("Important Birthday!"),
//         50..i32::MAX => println!("Important Birthday!"),
//         _ => println!("Not an important Birthday!"),
//     };
// }

// fn main() {
//     let my_age = 18;
//     let voting_age = 18;
//     match my_age.cmp(&voting_age) {
//         Ordering::Greater => println!("Congrats!! You are eligible to vote!"),
//         Ordering::Less => println!("Sorry, You are not eligible to vote."),
//         Ordering::Equal => println!("Yay!! You just earned the right to vote."),
//     };
// }

//Arrays

// fn main() {
//     let arr_1 = [2,3,4,5];
//     println!("1st: {}", arr_1[0]);
//     println!("Length: {}", arr_1.len());
// }

//Loops

fn main() {

}

