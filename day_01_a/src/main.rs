use adventio;
fn main() {
    let (mut list1, mut list2) = adventio::read_inputs_day_01("../input.txt".to_string());
    list1.sort();
    list2.sort();
    let distances = get_distances(list1, list2);
    let sum: isize = distances.iter().sum();
    println!("Sum = {sum}");
}


fn get_distances(list1: Vec<isize>, list2: Vec<isize>)-> Vec<isize>{
    let mut distances: Vec<isize> = Vec::new();
    if list1.len() != list2.len(){
        panic!("lists are not equal")
    }
    for i in 0..list1.len(){
        let a = list1[i];
        let b = list2[i];
        distances.push(isize::abs(a - b));
    }
    distances
}
