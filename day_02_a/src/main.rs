
use adventio;

fn main() {
    let f = adventio::read_inputs_day_02("input.txt".to_string());
    let res = get_safe(f); 
    println!("{res} is the ammount of safe reports");

}

fn get_safe(table: Vec<Vec<isize>>) -> isize{
    let mut out: isize = 0;
    for row in table{
        let mut safe = true;
        for i in 0..row.len()-1{
            let distance = isize::abs(row[i] - row[i + 1]);
            if distance > 3 || distance == 0{
                safe = false;
                break;
            }
        }
        if safe && (row.is_sorted() || row.is_sorted_by(|a,b| b < a)){
            println!("{:?} is safe", row);
            out += 1;
        }
        else{
            println!("{:?} is unsafe", row);
        }
    }
    out
}




#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn works(){
        let input = adventio::read_inputs_day_02("test.txt".to_string());
        let result: isize = get_safe(input);
        assert_eq!(result, 2);
    }
}
