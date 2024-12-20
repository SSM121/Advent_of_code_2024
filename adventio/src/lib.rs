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

pub fn read_inputs_day_02(filename: String)-> Vec<Vec<isize>>{
    let mut out: Vec<Vec<isize>> = Vec::new();
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines = data.lines();
    for line in lines{
        let words = line.split_whitespace();
        let mut l: Vec<isize> = Vec::new();
        for word in words{
            l.push(word.parse::<isize>().unwrap());
        }
        out.push(l);
    }
    out
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn day01(){
        let (list1, list2) = read_inputs_day_01("test1.txt".to_string());
        let goal1 = vec![3, 4, 2, 1, 3, 3,];
        let goal2 = vec![4, 3, 5, 3, 9, 3,];
        assert_eq!(goal1, list1);
        assert_eq!(goal2, list2);
    }
    #[test]
    fn day02(){
        let out = read_inputs_day_02("test2.txt".to_string());
        let goal = vec![    vec![7, 6, 4, 2, 1,],
                            vec![1, 2, 7, 8, 9,],
                            vec![9, 7, 6, 2, 1,],
                            vec![1, 3, 2, 4, 5,],
                            vec![8, 6, 4, 4, 1,],
                            vec![1, 3, 6, 7, 9,],];
        assert_eq!(out, goal);
    }
}
