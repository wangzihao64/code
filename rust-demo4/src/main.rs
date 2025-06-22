

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("{}", hello);
    println!("{}", world);

    let mut s = String::from("wzh");
    let word=first_word(&s);

    println!("{}", s);
    println!("{}",word);

    let a = [1,2,3,4,5,6,7,8];
    let slice = & a[1..3];
    assert_eq!(slice,&[2,3]);

    let mut s= "Hello";
    s="world";
    println!("{}",s);

    let mut s=String::from("hello");
    s.push_str(", world");
    s.insert(0,'w');
    println!("{}",s);
    let news = s.replace("whllo","wzh");
    println!("{}",news);

    let mut s=String::from("hello");
    s.replace_range(0..1,"w");
    s.pop();
    println!("{}",s);

    let mut s=String::from("我是中国人");
    let mut news=s.remove(0);
    println!("{}",s);
    println!("{}",news);

    let mut s=String::from("wzh");
    s.truncate(1);
    println!("{}",s);

    let rust=String::from("rust");

    let hello = String::from("hello");
    let result = hello + &rust;
    println!("{}",result);
    let result1=result+"111";
    println!("{}",result1);


    let s1="hello";
    let s2="rust";
    let s=format!("{}{}",s1,s2);
    println!("{}",s);

    let c = String::from("hello");
    for s in c.chars(){
        println!("{}",s);
    }


    let tup = (1,2,3);
    let (x,y,z)=tup;
    println!("{}{}{}",x,y,z);


    let user1=User {
        username:String::from("wzh"),
        email:String::from("185892713@qq.com"),
        phone:13390761696
    };

    let user2=build_user(String::from("wzh"),String::from("wzh"));
    println!("{}",user1.phone);

    let user3=User {
        username:String::from("yhn"),
        ..user1
    };

    println!("{}",user3.username);


    struct Color(i32,i32,i32);
    let black=Color(0,0,0);
    let r1=Rectangle{
        width:30,
        height:30
    };
    print!("{:?}",r1);
    let x=Color1::Black;

    println!("{:?}",x);
    let c1=PockerCard::Clubs(5);
    println!("{:?}",c1);


    let some_number=Some(5);
    let a = [1,2,3,4,5];

    let a:[i32;5]=[1,2,3,4,5];

    let a1:[String;8]=std::array::from_fn(|_i|String::from("rust is good"));
    print!("{:#?}",a1);


    let arr=[1,2,3];
    let slice:&[i32]=&arr[1..2];
    


}
#[derive(Debug)]
enum Color1{
    Black,
    White,
    Red
}
#[derive(Debug)]
enum PockerCard{
    Clubs(i32),
    Spades(i32),
    Diamond(i32),
    Hearts(i32),
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn first_word(s : &String) -> &str {
    &s[..1]
}
struct User{
    username:String,
    email:String,
    phone:i64
}
fn build_user(email:String,username:String)->User{
    User {
        username,
        email,
        phone:1
    }
}