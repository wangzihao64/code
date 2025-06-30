use std::ffi::CString;
use reqwest::header::AUTHORIZATION;
// fn add<T:std::ops::Add<Output=T>>(a:T, b:T) -> T {
//     a + b
// }
// use std::fmt::Display;
// fn create_and_point<T>()where T:From<i32>+Display {
//     let a:T = 100.into();
//     println!("a = {}",a);
// }
//
// struct Point<T>{
//     x:T,
//     y:T,
// }
//
// impl<T> Point<T>{
//     fn test(&self)->&T{
//         &self.x
//     }
// }
//
// struct Point1<T,U>{
//     x:T,
//     y:U,
// }
//
// impl<T,U> Point1<T,U>{
//     fn mixup<V,W:Copy>(self,other:&Point1<V,W>)->Point1<T,W>{
//         Point1{
//             x:self.x,
//             y:other.y.clone(),
//         }
//     }
// }
//
// impl Point1<f32,f32>{
//     fn distance_from_origin(&self)->f32{
//         println!("{}",self.x.powi(2));
//         println!("{}",self.y.powi(2));
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn display_array<T:std::fmt::Debug>(arr:&[T]){
//     println!("{:?}",arr);
// }
//
// const fn add1(a:usize,b:usize)->usize{
//     a+b
// }
//
// const RESULT:usize=add1(1,2);
//
// struct Buffer<const N:usize>{
//     buf:[u8;N],
// }
//
// const fn computer_buffer_size(factor:usize)->usize{
//     factor*4
// }
//
//
// use reqwest;
// use tokio;
//
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
#[derive(Clone)]
struct Message {
    role: String,
    content: String,
    tool_call_id: Option<String>,
}

#[derive(Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct DeepSeekResponse_wzh {
    choices: Vec<Choice_wzh>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct Choice_wzh {
    message: MessageResponse_wzh,
}

#[derive(Deserialize)]
struct MessageResponse {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MessageResponse_wzh {
    content:String,
    tool_calls:Vec<ToolCall_wzh>,
    role:String,
}

#[derive(Deserialize)]
struct ToolCall_wzh{
    id:String,
    #[serde(rename = "type")]
    type_:String,
    function:Function_wzh,
}

#[derive(Deserialize)]
struct Function_wzh{
    name:String,
    arguments:String,
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let api_key="sk-1be052b3be9e43d09bbae419874e896f";
    let url="https://api.deepseek.com/v1/chat/completions";
    let mut messages_wzh=vec![
        Message{
            role: "user".to_string(),
            content: "杭州天气如何？".to_string(),
            tool_call_id: None,
        },
    ];
    let request_body=DeepSeekRequest{
        model:"deepseek-chat".to_string(),

        messages:messages_wzh.clone(),
    };
    let reposonse=client
        .post(url)
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;
    //println!("{:#?}", String::from_utf8(reposonse.bytes().await?.to_vec()));


    let json:DeepSeekResponse_wzh=reposonse.json().await?;
    
    let message=&json.choices[0].message;
    
    println!("{:#?}", message.content);
    
    let tool=message.tool_calls.get(0).unwrap();
    
    
    let new_message:Message=Message{
        role:message.role.to_string(),
        content:message.content.to_string(),
        tool_call_id:None,
    };
    messages_wzh.push(new_message);
    

    
    messages_wzh.push( Message{
        role: "tool".to_string(),
        tool_call_id:Some(tool.id.to_string()),
        content: "24℃".to_string(),
    });
    
    
    
    let request_body=DeepSeekRequest{
        model:"deepseek-chat".to_string(),
    
        messages:messages_wzh.clone(),
    };
    let reposonse=client
        .post(url)
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

   
    
    let json:DeepSeekResponse_wzh=reposonse.json().await?;
    
    let message=&json.choices[0].message;
    
    println!("{:#?}", message.content);
    
    
    
    Ok(())

}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     let response = reqwest::get("https://api.deepseek.com/v1")
//         .await?
//         .text()
//         .await?;
//
//     println!("响应内容:\n{}", response);
//
//     Ok(())
// }

// // fn main() {
// //
// //     const SIZE:usize=computer_buffer_size(1024);
// //
// //     let buffer = Buffer::<SIZE>{
// //         buf:[0;SIZE],
// //     };
// //
// //     println!("{}", add(1,2));
// //
// //     create_and_point::<i32>();
// //
// //     let integer = Point{x:5,y:10};
// //     println!("{}",integer.x);
// //
// //     let integer2 = Point{x:5,y:10};
// //     println!("{}",integer2.test());
// //
// //     let interger3=Point1{x:6,y:11.0};
// //     let interger4=Point1{x:"Hello",y:'c'};
// //     interger3.mixup(&interger4);
// //     println!("{}",interger4.y);
// //
// //     let interger5=Point1{x:2.0,y:1.0};
// //     let x=interger5.distance_from_origin();
// //     println!("{}",x);
// //
// //     let arr:[i32;3]=[1,2,3];
// //     display_array(&arr);
// //
// //     let arr:[i32;2]=[1,2];
// //     display_array(&arr);
// //
// //
// //     println!("{}",RESULT);
// //
// //
// //
// //
// // }
