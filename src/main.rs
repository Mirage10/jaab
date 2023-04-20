
use rand::Rng;
use std::*;

fn main() {
    println!("Hello, world!");
   
   



let mut rng = rand::thread_rng();
println!("init");
let mut v = vec![];
let n:u32= 100000000;
for _i in 0 ..n {
   v.push( rng.gen_range(0..n));
   
};
//let mut vec: Vec<i64> =(0..5000000000).map(|i|  i%337).collect();
println!("endinit");
println!("start");
v.sort_unstable();
println!("end");

//let mut rng = rand::thread_rng();
   
   
let mut buffer = String::new();
    let a = io::stdin().read_line(&mut buffer);
    let ee = a.unwrap();

 println!("ee{}",ee);

 //   match a{
 //     Ok(pp)=>println!("ok{}",pp),
      
 //     Err(ss)=> println!("err{}",ss)
 //   }
    println!("{}", buffer);

}