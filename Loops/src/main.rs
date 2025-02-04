fn main() {
    print!("{}",6);
}

fn exp() -> i32{
    let a = 4;
    let b =6;
    let result = if 1==2 {
        return 10
    }
    else{
        return 20+10
    };
}




fn looping(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}