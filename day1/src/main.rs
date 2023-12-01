use std::fs;


static TEST: bool = false;
static INPUTPATH: &str = if TEST {"./test.txt"} else {"./input.txt"};

fn main() {
    // part_1();
    part_2();
}



#[allow(dead_code)]
fn part_1(){
    let input = fs::read_to_string(INPUTPATH).expect(&format!("Missing {}", INPUTPATH));

    fn get_calibration_value(strsl: &str)->u32{
        
        //get first digit from the left
        let mut digits = strsl.chars().filter(|x| x.is_ascii_digit());

        let first_digit = digits.clone().next().and_then(|x|x.to_digit(10)).expect("First Digit Parse Err");

        let last_digit = digits.next_back().and_then(|x|x.to_digit(10)).expect("Last Digit Parse Err");


        //get first digit from the right
        first_digit * 10 + last_digit
    }
    println!("input len = {}", input.lines().count());

    let res = input.lines().map(get_calibration_value).fold(0, |acc, e|{
        // println!("dbg: {}", e);
        acc + e
    });

    //.expect("should work!");

    println!("Answer: {}",  res)
}




static SPELLED_OUT : [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

#[allow(dead_code)]
fn part_2(){
    let input = fs::read_to_string(INPUTPATH).expect(&format!("Missing {}", INPUTPATH));

    fn get_calibration_value(strsl: &str)->u32{
        //get first digit from the left
        let first_digit = (||{
            let mut cpy = strsl.clone();

            while cpy.len() > 0 {
                let first_char = cpy.chars().next().unwrap();
                if first_char.is_ascii_digit(){
                    return first_char.to_digit(10);
                }

                for (idx, num) in SPELLED_OUT.iter().enumerate(){
                    if cpy.starts_with(num){
                        return Some(idx as u32 + 1);
                    }
                }
                //cut off the first char
                cpy = cpy.split_at(1).1
            }

            None


        })().expect("First num parse err :(");

        let last_digit = (||{
            let mut cpy = strsl.clone();

            while cpy.len() > 0 {

                let last_char = cpy.chars().next_back().unwrap();
                if last_char.is_ascii_digit(){
                    return last_char.to_digit(10);
                }

                for (idx, num) in SPELLED_OUT.iter().enumerate(){
                    if cpy.ends_with(num){
                        return Some(idx as u32 + 1);
                    }
                }
                //cut off the last char
                cpy = cpy.split_at(cpy.len()-1).0;
            }

            None
        })().expect("Last num parse err :(");


        //get first digit from the right
        first_digit * 10 + last_digit
    }
    println!("input len = {}", input.lines().count());

    let res = input.lines().map(get_calibration_value).fold(0, |acc, e|{
        println!("dbg: {}", e);
        acc + e
    });

    //.expect("should work!");

    println!("Answer: {}",  res)
}
