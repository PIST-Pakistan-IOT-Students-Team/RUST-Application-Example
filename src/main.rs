// fn main(){

//     let coin = Coin::Penny;

//     let value = value_of_coin(coin);

//     println!("Coin {:?}",value)
// }
// // #[derive(Debug)]
// // enum Coin {
// //     Penny,
// //     Nickel,
// //     Dime,
// //     Quarter,
// // }

// // fn value_of_coin(a : Coin) ->u8{
// //     match a {
// //         Coin::Penny => { 
// //             println!("Lucky Penny");
// //             1
// //         },
// //         Coin::Nickel => 5,
// //         Coin::Dime => 10,
// //         Coin::Quarter => 25,
// //     }
// // }


// // fn main(){
// //     let x:u8 = 3;

// //     match x {
// //         1 => println!("one"),
// //         2 => println!("two"),
// //         3 => println!("three"),
// //         _ => print!("None")
// //     }
// // }


#[derive(Debug)]
struct Student{
    Name: String,
    RollNo: u8,
    Gender: Gender,
    Timing: Timing,
    Course: Course,
}
#[derive(Debug)]
enum Gender{
    Male,
    Female
}
#[derive(Debug)]
enum Timing{
    Morning,
    Evening
}
#[derive(Debug)]
enum Course{
    IOT,
    AI
}

fn main(){

    let student1 = Student{
        Name : String::from("Jamil"),
        RollNo: 001,
        Gender: Gender::Male,
        Timing: Timing::Morning,
        Course: Course::IOT
    };
    let student2 = Student{
        Name : String::from("Zain"),
        RollNo: 002,
        Gender: Gender::Male,
        Timing: Timing::Evening,
        Course: Course::IOT
    };
    let student3 = Student{
        Name : String::from("Mehak"),
        RollNo: 003,
        Gender: Gender::Female,
        Timing: Timing::Morning,
        Course: Course::IOT
    };
    let student4 = Student{
        Name : String::from("ID Khan"),
        RollNo: 004,
        Gender: Gender::Male,
        Timing: Timing::Evening,
        Course: Course::IOT
    };
    let student5 = Student{
        Name : String::from("Zuhair"),
        RollNo: 005,
        Gender: Gender::Male,
        Timing: Timing::Morning,
        Course: Course::AI
    };

    let std_arr = [student1,student2,student3,student4,student5];

    for i in std_arr.iter(){
        match i.Timing{
            Timing::Morning => match i.Course{
                Course::IOT => println!("{:?} you are in",i.Name ),
                Course::AI => println!("{:?} this class is only for IOT Studets",i.Name)
            } ,
            Timing::Evening => println!("{:?} this class is only for Morning Student",i.Name)
        }
    }
 }



// &str String
// bool
// int 
// u8 i8
// u16 i16
// u32 i32
// u64 i64
// usize isize

//  let arr = [ [] , ]
//  arr[0][0] 

// ( str , bool , int , () , [])






// fn main(){
    // let x = 5;
    // let y = 7;
    // if x > y{
    //     println!("Print y {}",y)
    // }else{
    //     println!("Print x {}",x)
    // }
    //     let mut a = 0;
    // loop {
    //      println!("Loop");
    //     a = a+1;
    //     if a == 10 {
    //         break
    //     }
    // }


    // for i in 0..10{
    //     println!("{}",i)
    // }

    // let mut a = 0;
    // let flag = true;

    // while flag{
    //     a = a+1;
    //     println!("{}",a);
    //     if a == 10 {
    //         break
    //     }
    // }
// }

// >=
// <=
// ==
// !=
// Add function
// fn add(x:i32,y:i32)-> i32{
//     x+y
// }
/*
    Commint
    */