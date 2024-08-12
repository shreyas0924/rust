use std::{ f64::consts::PI, fs };

//Monomorphization
#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}
#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

//Traits
pub trait Summary {
    fn get_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn get_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}'s tweet - {}", self.username, self.content)
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

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
    star();
    strings();
    mutability();

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

    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let largest = largest(&list);
    println!("The largest number is {}", largest);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    match integer {
        Option_i32::Some(i) => println!("{}", i),
        Option_i32::None => println!("None"),
    }
    match float {
        Option_f64::Some(f) => println!("{}", f),
        Option_f64::None => println!("None"),
    }

    let tweet = Tweet {
        username: String::from("shreyas"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };

    println!("Tweet Summary - {}", tweet.summarize());
    print!("Author is {}", tweet.get_author())
}
