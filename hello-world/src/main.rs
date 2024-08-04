fn star() {
    let x = 5;
    let y = 5;

    for i in 0..x {
        for _ in i..y {
            print!("{}", "* ");
        }
        print!("\n");
    }

    for i in (0..x).rev() {
        for _ in i..y {
            print!("* ");
        }
        println!();
    }
}

fn strings() {
    let greeting = String::from("Hello World");
    println!("{}", greeting);

    let char = greeting.chars().nth(1);

    match char {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }

    print!("{:?}", char);
}

fn main() {
    star();
    strings()
}
