

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