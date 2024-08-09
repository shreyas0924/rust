use std::{ f64::consts::PI, fs };

fn star() {
    let x = 5;
    let y = 5;

    for i in 0..x {
        for _ in i..y {
            println!("{}", "* ");
        }
        println!("\n");
    }

    for i in (0..x).rev() {
        for _ in i..y {
            println!("* ");
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

    println!("{:?}", char);
}

fn mutability() {
    let mut str = String::from("hello world");
    str.push_str("asd");
}

#[derive(Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn move_around(direction: Direction) {
    println!("{:?}", direction)
}
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn find_first_a(string: String) -> Option<i32> {
    for (index, char) in string.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    // star();
    // strings();
    // mutability();

    let mut s1 = String::from("HEllo world");
    let s2 = &mut s1;
    // println!("{}", s1); // owner is shifted to s2 and s1 is not valid
    println!("{}", s2);

    let north = Direction::NORTH;
    let south = Direction::SOUTH;
    move_around(north);
    move_around(south);
    move_around(Direction::EAST);
    move_around(Direction::WEST);

    let circle = calculate_area(Shape::Circle(5.0));
    println!("The area of circle is {}", circle);

    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => print!("Contents of the file - {}", content),
        Err(error) => print!("{}", error),
    }

    let some_char = String::from("ssssssssn");
    let ans = find_first_a(some_char);

    match ans {
        Some(idx) => println!("Index is {}", idx),
        None => println!("There's no 'a' "),
    }
}
