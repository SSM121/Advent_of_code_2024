use std::fs;

pub fn read_inputs_day_01(filename: String) -> (Vec<isize>, Vec<isize>){
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
