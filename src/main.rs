

fn main() {
    println!("Hello, world!");
   
   




println!("init");
let mut v = vec![];
let n:u32= 1000000000;
let _aa:u32=3;
for i in 0 ..n {
   v.push( i%337 );     //rng.gen_range(0..n)
   
};
//let mut vec: Vec<i64> =(0..5000000000).map(|i|  i%337).collect();
do_sort(v.clone());

//let mut rng = rand::thread_rng();
   
do_sort(v);

}



fn do_sort( mut v :Vec<u32>) {
    println!("endinit");
    println!("start");
    v.sort_unstable();
    println!("end");
}


//use std::collections::{HashMap, HashSet};

//let mut rng = rand::thread_rng();
//use rand::Rng;
//use std::*;

// fn dfs(graph: &HashMap<i32, Vec<i32>>, node: i32, visited: &mut HashSet<i32>, output: &mut Vec<i32>) {
//     if visited.contains(&node) {
//         return;
//     }
//     visited.insert(node);
//     if let Some(neighbors) = graph.get(&node) {
//         for &neighbor in neighbors {
//             dfs(graph, neighbor, visited, output);
//         }
//     }
//     output.push(node);
// }

// fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
//     let mut visited = HashSet::new();
//     let mut output = Vec::new();
//     for &node in graph.keys() {
//         dfs(graph, node, &mut visited, &mut output);
//     }
//     output.reverse();
//     output
// }

// fn main() {
//     let mut rng = rand::thread_rng();

//     let mut graph = HashMap::new();
//     graph.insert(1, vec![2, 3]);
//     graph.insert(2, vec![4, 5]);
//     graph.insert(3, vec![5]);
//     graph.insert(4, vec![6]);
//     graph.insert(5, vec![6]);
//     let sorted = topological_sort(&graph);
//     println!("{:?}", sorted);

//     let D = "hallo".bytes();


//     for d in D{
//       println!("{}",d)
//     }
// }

