fn test(s:String)->(){
    println!("{}",s);
}
fn test1(x:i32)->(){
    println!("{}",x);
}
fn main() {
    let mut s= String::from("hello");
    s.push_str(", world!");
    println!("{}",s);
    let s1=&String::from("hello");
    let s2=s1;
    println!("{}",s2);
    println!("{}",s1);
    let s=String::from("hello test");
    test(s);
    //这样会报错
    //println!("{}",s);
    let x=5;
    test1(x);
    println!("{}",x);
}
