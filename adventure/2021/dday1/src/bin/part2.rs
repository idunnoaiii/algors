use std::fs;

fn main() {
    let file = fs::read_to_string("./input2.txt").unwrap();

    let inputs = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut it = inputs.iter().peekable();

    for _ in 0..it.len() {
        let one = it.next();
        let two = it.peek();
        let three = it.peek();

        match (&one, &two, &three) {
            (Some(a), Some(&b), Some(&c)) => todo!(),
            _ => {}
        };
    }

    let mut counter = 0;
    // for item in inputs.windows(4) {
    //     if let [a, b, c, d] = item {
    //         let f = a + b +c;
    //         let t = b + c + d;

    //         if t > f {
    //             counter += 1;
    //         }
    //     } 
    //     else {

    //     };

    // }


    dbg!(counter);

}
