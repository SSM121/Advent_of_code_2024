use std::fs;

fn main() {
    let (mut list1, mut list2) = read_inputs("input.txt".to_string());
    list1.sort();
    list2.sort();
    let distances = get_distances(list1, list2);
    let sum: isize = distances.iter().sum();
    println!("Sum = {sum}");
}

fn read_inputs(filename: String) -> (Vec<isize>, Vec<isize>){
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines = data.lines();
    let mut list1: Vec<isize> = Vec::new();
    let mut list2: Vec<isize> = Vec::new();
    for line in lines{
        let mut words = line.split_whitespace();
        let x = words.next().unwrap().parse::<isize>().unwrap();
        list1.push(x); 
        let y = words.next().unwrap().parse::<isize>().unwrap();
        list2.push(y);   
    }
    (list1, list2)
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
