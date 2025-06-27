use std::process::id;
use std::str::FromStr;
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
enum IpAddr{
    V4,
    V6,
}

enum UsState{
    Alabama,
    Alaska,
}



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Action{
    Say(String),
    MoveTo(i32,i32),
    ChangeColor(i32,i32,i32),
}
enum Test{
    Quit,
    Move{x:i32,y:i32},
    ChangeColor(i32,i32,i32),
}

enum Message{
    Hello{id:i32}
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None=>None,
        Some(i) => Some(i+1),
    }
}

fn get_count_item(s:&str)->(u64,&str){
    let mut it = s.split(' ');
    let (Some(counter_str),Some(item))=(it.next(), it.next())
    else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count)=u64::from_str(counter_str)
    else {
        panic!("Can't parse integer: '{counter_str}'")
    };
    (count,item)
}

struct Point{
    x:i32,
    y:i32,
}

fn main() {
    let arr:&[u16] = &[114,244];
    if let [x,..]=arr{
        assert_eq!(x,&114);
    };
    if let &[..,y]=arr{
        assert_eq!(y,244);
    }

    let arr:&[u16]=&[];
    assert!(matches!(arr,[..]));



    assert_eq!(get_count_item("3 chairs"),(3,"chairs"));

    let p = Point{x:3,y:5};
    let Point{x,y}=p;
    assert_eq!(x,3);
    assert_eq!(y,5);


    let a =plus_one(None);
    println!("{:?}",a);
    let a=plus_one(Some(5));
    println!("{:?}",a);
    let a=plus_one(None);
    println!("{:?}",a);
    let a=plus_one(Some(3));
    println!("{:?}",a);
    let test=Test::Move{x:3,y:5};
    match test{
        other=> println!("other"),
    };

    let action = Action::Say(String::from("hello"));
    match action {
        Action::Say(s) => println!("Say"),
        Action::MoveTo(x,y) => println!("Move to ({},{})", x,y),
        _ => println!("Not a Action"),
    };

    let direction = Direction::NORTH;
    match direction {
        Direction::NORTH => println!("NORTH"),
        Direction::SOUTH | Direction::EAST => { println!("SOUTH AND EAST"); },
        _ => println!("WEST"),
    };
    let ip=IpAddr::V4;
    let res=match ip {
        IpAddr::V4 => "ipv4",
        _ => "ipv6",
    };
    println!("{:?}",res);

    let coin = Coin::Quarter(UsState::Alaska);

    let res=match coin{
        Coin::Penny => "Penny",
        Coin::Nickel => "Nickel",
        Coin::Dime => "Dime",
        Coin::Quarter(UsState::Alabama) => "1" ,
        Coin::Quarter(UsState::Alaska) => "2" ,
    };
    println!("{:?}",res);
    let v=Some(3);
    if let Some(3)=v {
        println!("v is 3");
    }
    #[derive(Debug)]
    enum MyEnum
    {
        Foo,
        Bar
    }

    let v=vec![MyEnum::Foo, MyEnum::Bar];

    for i in v {
        println!("{:?}",i);
    }


    let x = Some(5);
    match x {
        Some(n) => println!("值是 {}", n),
        None => {} // 什么都不做
    }

    let age=Some(30);
    println!("{:?}",age);
    if let Some(age)=age{
        println!("{:?}",age);
    }
    println!("{:?}",age);

    
    let x=Some(1);
    println!("{:?}",x);
    if let Some(n)=x{
        println!("{:?}",n);
    }
    println!("{:?}",x);

    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v=vec!['a','b','c'];
    for (index,value) in v.iter().enumerate() {
        println!("{}{}",index,value);
    }

    let x=(1,2,3);
    println!("{:?}",x);

    let x:Option<i32> = None;

    if let None = x {
        println!("None");
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'a'..='j' => println!("early ASCII letter1"),
        _ => println!("something else"),
    }


    let msg=Message::Hello {id:5};
    match msg{
        Message::Hello{id:id_var@3..=7}=>{
            println!("id:{}",id_var);
        }
        Message::Hello {id:id_varr@10..=12}=>{
            println!("id:{}",id_varr);
        }
        Message::Hello{id}=>{
            println!("id:{}",id);
        }
    }


}
