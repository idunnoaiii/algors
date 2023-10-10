use std::collections::HashMap;

/**
 * time: O(n)
 * space: O(n)
*/
fn _is_anagram_hashmap(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let mut memory = s.chars().fold(HashMap::<char, i32>::new(), |mut map, c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        map
    });

    for c in t.chars() {
        if let Some(v) = memory.get_mut(&c) {
            *v -= 1;
            if *v == 0 {
                memory.remove(&c);
            }
        } else {
            return false;
        }
    }

    return memory.len() == 0;
}


fn _their_solution(s: String, t : String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::<char, i32>::new();

    for (cs, ct) in s.chars().zip(t.chars()) {
        *(map.entry(cs).or_default()) += 1;
        *(map.entry(ct).or_default()) -= 1;
    }

    return map.into_values().all(|v| v == 0);
}


fn main() {
}
