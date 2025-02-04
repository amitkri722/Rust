fn main(){
    let x =5;
    println!("The value of x is: {}",x);
    let x=6;
    println!("The value of x is: {}",x);

    let guess:i32 = "42".parse().expect("Not a number!");
    println!("{}",guess);

    let tuple = (23,654,'t');
    let (x,z,y) = tuple;
    println!("{z}");
    helper();
}

fn helper(){
    println!("Hi from helper...");
}
