fn main() {
    let a=1;
    assert_eq!(a,1);
    let a:u8=255;
    let b=a.wrapping_add(20);
    println!("{}",a);
    let x=2.0;
    let y:f32=2.0;
    println!("{}",x);
    println!("{}",y);
    let abc:(f32,f32,f32)=(0.1,0.2,0.3);
    let xyz:(f64,f64,f64)=(0.1,0.2,0.3);
    println!("{}",(abc.0+abc.1).to_bits());
    println!("{}",(abc.2).to_bits());
    println!("{}",(xyz.0-xyz.1).to_bits());
    println!("{}",(xyz.2).to_bits());
    let x=(-42.0_f32).sqrt();
    if x.is_nan()
    {
        println!("NaN");
    }
    let i=1i32;
    println!("{:08b}",i);
    for j in 1..=5{
        println!("{}",j);
    }
    for i in 'a'..='z' {
        println!("{}",i);
    }
}
