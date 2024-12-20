use adventio;

fn main() {
    let (list1, list2) = adventio::read_inputs_day_01("../day_01_a/input.txt".to_string());
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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn works(){
        let (list1, list2) = adventio::read_inputs_day_01("test.txt".to_string());
        let result: isize = get_similarities(list1, list2).iter().sum();
        assert_eq!(result, 31);
    }
}
