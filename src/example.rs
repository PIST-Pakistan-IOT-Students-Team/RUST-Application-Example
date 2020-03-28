// fn main(){
//     let x:i8 = 3;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//          _ => println!("Your number is out of range"),
//     }
// }









// #[derive(Debug)]
// enum Shape{
//     Circle(u32),
//     Triangle(u32,u32,u32),
//     Rectangle(u32,u32,u32,u32),
//     Square(u32,u32,u32,u32)
// }

// impl Shape{
//     fn enum_data(&self) -> u32{
//         match self{
//             &Shape::Circle(radius) => return radius/(2*3),
//             &Shape::Triangle(x,y,z) => return x+y+z,
//             &Shape::Rectangle(a,b,c,d)=> return a+b+c+d,
//             &Shape::Square(a,b,c,d) => return a+b+c+d
//          }
//     }

// }

// fn main(){
//     let triangle = Shape::Triangle(40,50,80);
//     let circle = Shape::Circle(40);
//     let square = Shape::Square(20,20,20,20);
//     let rectangle = Shape::Rectangle(30,40,30,40);

//     println!("Circle Variant Return {}",circle.enum_data());
//     println!("Triangle Variant Return {}",triangle.enum_data());
//     println!("Square Variant Return {}",square.enum_data());
//     println!("Rectanlge Variant Return {}",rectangle.enum_data());   

// }



// #[derive(Debug)]
// enum BrandName{
//     Hp,
//     Dell(DellVariant),
//     Asus,
//     Lenovo
// }
// #[derive(Debug)]
// enum DellVariant{
//     Series1000,
//     Series2000,
//     Series3000,
//     Series4000,
//     Series5000,
//     Series6000
    
// }


// fn main(){
//     let laptop1 = BrandName::Lenovo;
//     let laptop2 = BrandName::Dell(DellVariant::Series3000);
//     let laptop3 = BrandName::Asus;
//     let laptop4 = BrandName::Dell(DellVariant::Series5000);

//     print_laptop(laptop1);
//     print_laptop(laptop2);
//     print_laptop(laptop3);
//     print_laptop(laptop4);
// }
// fn print_laptop(model:BrandName){

//     match model{
//         BrandName::Hp => println!("My laptop Brand is Hp"),
//         BrandName::Dell(series) => println!("My laptop Brand is Dell and Series is {:?}",series),
//         BrandName::Asus => println!("My laptop Brand is Asus"),
//         BrandName::Lenovo => println!("My laptop Brand is Lenovo")
//     }

// }



// use std::io;

// fn main() {

//     println!("Enter Number 1 to 5:");

//     let mut input =  String::new();
    
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let mut input:i8 = input.trim().parse().unwrap();

//     match input{
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         4 => println!("four"),
//         5 => println!("five"),
//         _ => println!("Your number is out of range"),
//     }
// }






// #[derive(Debug)]
// struct Student { 
//     name : String,
//     rollno: String,
//     gender: Gender,
//     timing: Timing,
//     course : Course,
// }

// #[derive(Debug)]
// enum Gender{
//     Male,
//     Female
// }

// #[derive(Debug)]
// enum Timing{
//     Morning,
//     Evening,
// }
// #[derive(Debug)]
// enum Course{
//     IOT,
//     AI,
//     CNC
// }

// fn main(){

//     let student1 = Student{
//         name:String::from("Jamil"),
//         rollno:String::from("0001"),
//         gender:Gender::Male,
//         timing: Timing::Morning,
//         course: Course::IOT,
//     };
//     let student2 = Student{
//         name:String::from("Zain"),
//         rollno:String::from("0002"),
//         gender:Gender::Male,
//         timing: Timing::Morning,
//         course: Course::IOT,
//     };
//     let student3 = Student{
//         name:String::from("Zuhair"),
//         rollno:String::from("0003"),
//         gender:Gender::Male,
//         timing: Timing::Morning,
//         course: Course::CNC,
//     };
//     let student4 = Student{
//         name:String::from("Mehak"),
//         rollno:String::from("0004"),
//         gender:Gender::Female,
//         timing: Timing::Evening,
//         course: Course::IOT,
//     };
//     let student5 = Student{
//         name:String::from("Muzaina"),
//         rollno:String::from("0004"),
//         gender:Gender::Female,
//         timing: Timing::Evening,
//         course: Course::IOT,
//     };

//     let std_arr = [student1,student2,student3,student4,student5];

//     for i in std_arr.iter(){
//         match i.timing {
//             Timing::Morning => match i.course{
//                 Course::IOT => println!("{} can Enter The Class Room",i.name),
//                 _=>println!("Sorry {} This Class is for only IOT Students",i.name)
//             }
//             Timing::Evening => println!("Sorry {} This Class for only Morning Students",i.name)
//         }
//     }

// }
