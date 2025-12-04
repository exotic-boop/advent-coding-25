fn main() {
    let data = "";
    let mut answer = 0;
    for pair in data.split(','){
        let mut ids = pair.split('-');
        /* First puzzle
        if let (Some(first_str), Some(second_str)) = (ids.next(), ids.next()) {
            if let (Ok(start), Ok(end)) = (first_str.parse::<i64>(), second.parse::<i64>()) {

                for i in start..=end {
                    let s = i.to_string();
                    if s.len() % 2 == 0 {  
                        let mid = s.len() / 2;
                        let (left, right) = s.split_at(mid);
                        if left == right {
                            answer += i;
                        }
        
                } 
            }   
        }
            */
        //second puzzle
        if let (Some(first_str), Some(second_str)) = (ids.next(), ids.next()) {
            if let (Ok(start), Ok(end)) = (first_str.parse::<i64>(), second_str.parse::<i64>()) {

            for i in start..=end {
                let s = i.to_string();
                let doubled = format!("{}{}", s, s);

                if doubled[1..doubled.len()-1].contains(&s) {
                    answer += i as i64;
                }    
            }
            }
        }
    }
    println!("anwser: {}", answer);
}