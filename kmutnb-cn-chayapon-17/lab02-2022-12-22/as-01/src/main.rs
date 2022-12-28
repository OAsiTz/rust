use std::io;

fn main() {
    println!("Enter a number of floor:");
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("What wrong with you!!");
        return;
        },
    };
    
    for i in 0 .. x {
        for k in 0 .. x {
            if i >= k {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}