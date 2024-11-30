use std::fmt;
use std::error::Error;
struct MyError{
    message:String,

}
trait Speak{
    fn speak(&self);
}
struct Dog;
impl Speak for Dog{
    fn speak(&speak){
        println!("moof");
    }
}
impl fmt::Display for MyError{
    fn fmt(&self,f:&mut::Formatter<'_>)->fmt::Result{
    write!(f,"{}",self.message)
    }
}
impl Error for MyError{}

enum Result<T,E>{
    Ok(T),
    Err(E),
}
struct Person{
    name:String,age:32,
}
enum Animal{
    DOG,CAT,
}
enum Option<T>{
    Some(T),
    None
}
fn main(){
    match divide(10,0){
        Ok(result)=>println!(":{}".result)
        Err(e)=>println!(":{}",e);
    }
    let arrff  = [1,2,3,4,5];
    let slice  :&i32 =&arr[1..4];
    let number = Some(10);
match number {
    Some(value) =>println!("values is :{}",value);
    None =>println!("no values");
}
    let person = Person{
        name:"pawan",
        age:20,
    }
    println!("person name :{} ,person age : {}",person.name,person.age)
let item = find_item();
match item{
    Some(i) =>printlnn!("found :{}",i);
    None  =>println!("item not found");
}
}
fn find_item()->Option<String>{
    Some(String::fro("item wrong"))
}
fn speak(animal:::Animal){
    match animal{
        Animal::Dog=>println!("woof"),
        Animal::Cat =>println!("meow");
    }
}
fn divide(a:i32,b:i32)->Result<i32,String>{
    if b == 0{
        Err(String::from("cannot divide by zero"));
    }
    else{
        Ok(a/b)
    }
}

fn read_file()->Result<String ,std::io::Error>{
    let contents  =std::fs::read_to_string("file.txt");
Ok(contents)
}

fn generate_error()->Result<(),Box<dyn Error>>{
    Err(Box::new(MyError{message:String::from("something went wrong")}))
}

fn greet<T:Speak>(item:T){
    item.speak();
}