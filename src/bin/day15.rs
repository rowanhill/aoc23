fn sum_of_hashes(input: &[u8]) -> usize {
    let mut sum = 0;
    let mut hash = 0;
    for &b in input {
        if b == b',' {
            sum += hash;
            hash = 0;
        } else if b == b'\n' {
            continue;
        } else {
            hash = ((hash + (b as usize)) * 17) % 256;
        }
    }
    sum += hash;
    sum
}

fn populate_hashmap(input: &[u8]) -> Vec<Vec<(String, usize)>> {
    let mut boxes = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let mut bytes = input.iter().peekable();
    while bytes.peek().is_some() {
        // Generate label
        let mut label = String::new();
        let mut label_hash = 0;
        loop {
            match bytes.peek() {
                Some(b'=') | Some(b'-') => break,
                _ => {},
            };
            let b = *bytes.next().unwrap();
            label.push(b as char);
            label_hash = ((label_hash + (b as usize)) * 17) % 256;
        }
        let is_assign = *bytes.next().unwrap() == b'=';
        if is_assign {
            let focal_length = (*bytes.next().unwrap() - b'0') as usize;
            let mut did_replace = false;
            for (l, fl) in boxes[label_hash].iter_mut() {
                if l == &label {
                    *fl = focal_length;
                    did_replace = true;
                    break;
                }
            }
            if !did_replace {
                boxes[label_hash].push((label, focal_length));
            }
        } else {
            boxes[label_hash].retain(|(l, _)| l != &label);
        }
        // Consume the ,
        bytes.next();
    }

    boxes
}

fn focusing_power(boxes: &[Vec<(String, usize)>]) -> usize {
    let mut sum_power = 0;
    for (i, cur_box) in boxes.iter().enumerate() {
        for (j, (_, focal_length)) in cur_box.iter().enumerate() {
            let lens_power = (i + 1) * (j + 1) * focal_length;
            sum_power += lens_power;
        }
    }
    sum_power
}

fn main() {
    let input = include_bytes!("../../input/day15");
    println!("Part 1: {}", sum_of_hashes(input));
    println!("Part 2: {}", focusing_power(&populate_hashmap(input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example1() {
        assert_eq!(sum_of_hashes(EXAMPLE), 1320);
    }

    #[test]
    fn example2() {
        assert_eq!(focusing_power(&populate_hashmap(EXAMPLE)), 145);
    }
}