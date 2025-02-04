fn main() {
    println!("Hello, world!");

    let x =another_function();
    println!("{}",x);
}

fn another_function() -> i32 {
    println!("Another function.");
    7
}