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

    let x=5;
    let y=&x;
    assert_eq!(x,5);
    assert_eq!(*y,5);

    let s1=String::from("hello111");
    let len=calculate_length(&s1);
    println!("{}",len);
    println!("{}",s1.capacity());

    let mut s=String::from("hello222");
    let r1=&mut s;
    r1.push('1');
    println!("{}",r1);
    println!("{}",s);

    let r2=&mut s;
    println!("{}",r2);

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{}",r2);
    let r3 = &mut s; // 大问题
    println!("{}",r3);
    


}
fn calculate_length(s : & String ) ->usize{
    println!("{}",s.capacity());
    s.len()
}