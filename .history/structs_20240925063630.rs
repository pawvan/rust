

struct Point{
    x:f64,
    y:f64
}
 
 imp Point{
    fn distance(&self,other:&Point)->f64{
        let dx = self.x-other.x;
let dy=self.y-other.y;
(dx.powi(2)+dy,powi(2)).sqrt()
    
    }
 }

 imp Point{

    fn origin()->Point{
        Point {x:0.0,y:0.0}
    }
 }

 //tuple struct

 struct Color(u8,u8,u8);
 let black =Color(0,0,0,);

//unit-like struct
 struct Marker;
