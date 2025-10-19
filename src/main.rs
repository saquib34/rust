// fn main(){
//     println!("{}", is_even(4));
// }

// fn is_even(n:i32)-> bool{
//     if n%2==0{
//         return true;
//     }
//     return false;
// }

// write a function of fibbanacci series

// fn main(){
//     let x : u32 = 10;
//     println!("{}",fib(x));
// }

// fn fib(num:u32)-> u32{
//     let mut first=0;
//     let mut second=1;
//     if num == 0{
//         return  first;
//     }

//     if num == 1{
//         return second;
//     }
//     for _  in 0..(num-1) {
//         let temp = second;
//         second = first + second;
//         first = temp;
//     }
//     return second;
// }

// fn main(){
//     let name=String::from("saquib");
//     let len=get_str_len( name);
//     println!("length of string is {}",len);
// }

// fn get_str_len(str:String)->usize { 
//     return  str.chars().count();
// }

struct User {
    first_name: String,
    last_name: String,
    age:u32,
}

fn main(){
    let user=User{
        first_name:String::from("saquib"),
        last_name:String::from("Daiyan"),
        age:22,
    };
    println!("{}",user.first_name);
}