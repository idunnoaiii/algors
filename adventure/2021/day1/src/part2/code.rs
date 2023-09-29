use std::fs;

pub fn do_action() -> i32 {
    let file = fs::read_to_string("./src/part2/input.txt")     ;
    i32::min_value()
}


#[cfg(test)]
mod test {

    #[test]
    fn case1() {
        assert_eq!(super::do_action(), 0);
    }
}
