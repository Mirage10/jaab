

use inline_python::python;
fn main() {
    println!("Hello, world!");
    //let a  = (4,5,6);
    python! {
        print("ii")

}


let mut v = vec![3, 4, 5];

println!("init");
for i in 0..100000000{
   v.push(i%337);
};

println!("endinit");
println!("start");
v.sort_unstable();
println!("end");

}