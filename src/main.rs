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

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    // 关联函数
    fn new(width: u32, height: u32) -> Rectangle{
        Rectangle{width, height}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect = Rectangle::new(30, 50);
    println!("rect area is {}", rect.area());
}