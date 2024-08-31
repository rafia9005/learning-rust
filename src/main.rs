// import example from ''
mod example;

// use example
use example::hello;
use std::string::String;

//main example
fn main() {
    //hello world
    println!("Hello, world!");

    // variable
    let variable = "ini variable";
    println!("ini variable {}", variable);

    // function implement
    ini_function("test");

    // class implement
    let data = Data::new("Alice", 30);
    // show data from class
    data.greet();

    //function from import
    hello()
}

// function example
fn ini_function(value: &str) {
    println!("ini adalah function {}", value)
}

// data struct
struct Data {
    name: String,
    age: u32,
}

// class
impl Data {
    fn new(name: &str, age: u32) -> Data {
        Data {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("ini class");
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}
