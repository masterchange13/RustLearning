// todo demo1
// mod Animal;
// use Animal::Animal as OtherAnimal;
//
// struct Dog;
// impl OtherAnimal for Dog {
//     fn call(&self) {
//         println!("Woof");
//     }
// }
//
// fn main() {
//     let dog = Dog;
//     dog.call();
// }


// todo 子目录的文件调用
// 需要在子目录声明mod文件，对需要调用的文件进行声明
// mod animal2;
// use crate::animal2::demo::Animal;
//
// struct dog{
//
// }
// impl Animal for dog {
//     fn eat(&self) {
//         println!("dog eat");
//     }
// }
//
// fn main() {
//    let dog = dog{};
//     dog.eat();
// }


// todo demo3
// 关联函数学习

// struct Rectangle{
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle{
//     // 关联函数
//     fn new(width: u32, height: u32) -> Rectangle{
//         Rectangle{width, height}
//     }
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let rect = Rectangle::new(30, 50);
//     println!("rect area is {}", rect.area());
// }
//


// todo demo4
// enum learning

// enum children {
//     Boy,
//     Girl,
// }
//
// fn main() {
//     let child = children::Boy;
//     match child {
//         children::Boy => println!("this is a boy"),
//         children::Girl => println!("this is a girl"),
//     }
//
// }


// todo demo5
// struct learning

// #[derive(Debug)]
// struct Point{
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     let point = Point{x: 1, y: 2};
//     println!("{:?}", point);
// }


// todo demo6
// Ok Err learning

// use std::fmt::Error;
// use std::sync::Arc;
//
// fn main() {
//     let result: Result<i32, Error> = Ok(1);
//     match result {
//         Ok(num) => println!("num is {}", num),
//         Err(e) => println!("error is {}", e),
//     }
//
//     let result2: Result<i32, Error> = Err(Error);
//     match result2 {
//         Ok(num) => println!("num is {}", num),
//         Err(e) => println!("error is {}", e),
//     }
// }


// todo demo7
// Result learning
// result is a kind of enum, and u should match it, and use Ok or Err to set the value and get value

// use std::fmt::Error;
// use std::sync::Arc;
//
// fn main() {
//     let result: Result<i32, Error> = Ok(1);
//     match result {
//         Ok(num) => println!("num is {}", num),
//         Err(e) => println!("error is {}", e),
//     }
//
//     /** T is a type parameter, and it is server for Ok, so u should use Ok to set the value,
//         but don't use it when you set it as a type parameter
//         when you try to match it, just use data type, instead of real data
//     **/
//     let result2: Result<i32, Error> = Ok(1);
//     match result2 {
//         Ok(i32) => println!("Good"),
//         Err(e) => println!("error is {}", e),
//     }
// }