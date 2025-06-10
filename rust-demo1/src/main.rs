struct A
{
 AA:i32
}
fn main()
{
    let mut a=1;
    let _b=1;
    let (a1,mut b1):(bool,bool)=(true,false);
    println!("{}",a);
    println!("{},{}",a1,b1);
    let (c,d,e,f,AA);
    (c,d)=(1,2);
    (e,_,f)=(3,4,5);
    A {AA}=A{AA:5};
    println!("{}",AA);
    println!("{}{}",e,f);
    let x=5;
    let x=x+1;
    {
        let x=x*2;
        println!("{}",x);
    }
    println!("{}",x);
}
