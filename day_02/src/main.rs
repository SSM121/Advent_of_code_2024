use adventio;

fn main() {
    let (list1, list2) = adventio::read_inputs("../input.txt".to_string());
//    let (list1, list2) = adventio::read_inputs("test.txt".to_string());
    let sims = get_similarities(list1, list2);
    let sum: isize  = sims.iter().sum();
    println!("{sum}");
}

fn get_similarities(list1: Vec<isize>, list2: Vec<isize>)-> Vec<isize>{
    let mut similarities: Vec<isize> = Vec::new();
    if list1.len() != list2.len(){
        panic!("Lists are different lengths");
    }
    for num in list1{
        let mut sims: isize = 0;
        for n in &list2{
            if *n == num{
            sims +=1;
            }
        }
        similarities.push(num * sims);
    }
    similarities
}

