use std::io;

    fn main() {
        println!("Enter a number of reverse floor:");
    
        let mut x = String::new();
        io::stdin().read_line(&mut x)
            .expect("Failed to read line");
    
        let x: i32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("what wrong with you!!");
                return;
            },
    };
    for i in 0..x {
        for _k in 0..i {
            print!(" ");
        }
        for _j in 0..2 * (x - i - 1) + 1 {
            print!("*");
        }
        println!("");
    }
}