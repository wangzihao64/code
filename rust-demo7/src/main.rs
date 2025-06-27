
struct Circle{
    x:f64,
    y:f64,
    radius:f64,
}
impl Circle{
    fn new(x:f64,y:f64,radius:f64)->Circle{
        Circle{
            x:x,
            y:y,
            radius:radius,
        }
        
    }
    fn area(&self)->f64{
        std::f64::consts::PI*self.radius*self.radius
    }
}
fn main() {
    let react1=Circle::new(3.0, 2.0, 3.0);
    println!("Area = {}",react1.area());
    //这个和new方法没关系，不用new方法也可以用这个初始化
    let reactor2=Circle{x:3.0,y:2.0,radius:4.0};
}
