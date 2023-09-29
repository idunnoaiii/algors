pub fn do_action(inputs: Vec<u16>) -> i32 {
    let mut count = 0;

    for (idx, _) in inputs.iter().enumerate() {
        if idx == inputs.len() - 1 {
            break;
        }
        if inputs[idx + 1] > inputs[idx] {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod test {

    use std::fs;

    #[test]
    fn case1() {
        let file = fs::read_to_string("./src/part1/input.txt").unwrap();
        let inputs = file
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<u16>>();

        let actual = super::do_action(inputs);
        assert_eq!(actual, 1521);
    }
}
