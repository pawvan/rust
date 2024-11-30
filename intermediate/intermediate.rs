use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

struct Person {
    name: String,
    age: i32,
}

fn main() {
    match divide(10, 0) {
        MyResult::Ok(result) => println!("Result: {}", result),
        MyResult::Err(e) => println!("Error: {}", e),
    }

    let arrff = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arrff[1..4];
    println!("Slice: {:?}", slice);

    let number = Some(10);
    match number {
        Some(value) => println!("Value is: {}", value),
        None => println!("No value"),
    }

    let person = Person {
        name: String::from("Pawan"),
        age: 20,
    };
    println!("Person name: {}, Person age: {}", person.name, person.age);

    let item = find_item();
    match item {
        Some(i) => println!("Found: {}", i),
        None => println!("Item not found"),
    }

    let dog = Dog;
    greet(dog);
}

fn find_item() -> Option<String> {
    Some(String::from("Item found"))
}

fn divide(a: i32, b: i32) -> MyResult<i32, String> {
    if b == 0 {
        MyResult::Err(String::from("Cannot divide by zero"))
    } else {
        MyResult::Ok(a / b)
    }
}

fn greet<T: Speak>(item: T) {
    item.speak();
}
