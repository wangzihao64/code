fn main() {

    let flag=true;
    let num=if flag {
        5
    }
    else {
        6
    };
    println!("{}",num);

    let a=[1,2,3];
    for (i,v) in a.iter().enumerate() {
        println!("{}{}",i,v);
    }
    
    let mut n = 0;
    while (n<5){
        println!("{}",n);
        n=n+1;
    }
    println!("运行结束");
}
