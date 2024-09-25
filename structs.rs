struct Point{
    x:f64,
    y:f64
}
 struct User{
     username:String,
     count_in:u64,
     active:bool,
     email:String
 }
 imp Point{
    fn distance(&self,other:&Point)->f64{
        let dx = self.x-other.x;
let dy=self.y-other.y;
(dx.powi(2)+dy.powi(2)).sqrt()
    
    }
 }
trait Summary{
    fn origin(&self)->String
}
 imp Point{

    fn origin()->Point{
        Point {x:0.0,y:0.0}
    }
 }
     struct Color(u8,u8,u8);
 let black =Color(0,0,0,);

imp Point {
    fn distance(&self,&other:&Point)>f64{
    let dx = self.x -other.x;
let dy = self.y-other.y;
(dx.powi(2)+dy.powi(2).sqrt()
 }

}
 struct Point{
     x:0;
     y:0;
 }
imp Point{
    fn origin()->Point{
  Point {x:0,y:0}; 
    }
 }
struct Color(u8,u8,u8);
let black =Color(0,0,0);

fn main(){
    let user =User{
        username:"pawan",
        active:true,
        count_in:5,
        email:"pawanpediredla@gmail.com"
    }
        let user2=User{
        email:String::from("pawanpediredla@gmail.com"),
        ...user1
        }
    println!("{}",user.username);
Struct Color(u8,u8,u8);
let black =Color(255,255,255);
}
imp Summary for User {
    fn origin(&self)->String{
    format!("{} ,({})", self.username,self.email);
    }
}
