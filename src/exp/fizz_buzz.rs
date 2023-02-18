// refactor: pattern matching, enums to build the result

pub fn list(num: &i32) {
    for n in 1..*num {
        let mut result = String::new();

        if n % 3 == 0 {
            result.push_str("Fizz");
        }

        if n % 5 == 0 {
            result.push_str("Buzz");
        }

        if result.len() == 0 {
            println!("{n}")
        } else {
            println!("{result}")
        }
    }
}
