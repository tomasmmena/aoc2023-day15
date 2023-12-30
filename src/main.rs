use std::env;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Lense {
    label: String,
    focal_lenght: usize
}

#[derive(Debug)]
struct LenseArray {
    boxes: [Vec<Lense>; 256]
}

impl LenseArray {
    
    fn add_lense(&mut self, lense: Lense) {
        let box_num = hash(&lense.label);
        for l in self.boxes[box_num as usize].iter_mut() {
            if l.label == lense.label {
                l.focal_lenght = lense.focal_lenght;
                return;
            }
        }
        self.boxes[box_num as usize].push(lense);
    }

    fn remove_lense(&mut self, label: String) {
        let box_num = hash(&label);
        self.boxes[box_num as usize] = self.boxes[box_num as usize]
            .iter()
            .filter(|l| l.label != label)
            .map(|l| l.to_owned())
            .collect();
    }

    fn get_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, b)| b
                .iter()
                .enumerate()
                .map(move |(j, l)| (i + 1) * (j + 1) * l.focal_lenght)
            )
            .sum()
    }

}

fn hash(input: &str) -> u8 {
    let mut accum: u16 = 0;
    for byte in input.as_bytes() {
        accum = ((accum + *byte as u16) * 17) % 256;
    }
    accum as u8
}

fn main() {

    let path = env::args().nth(1).expect("Missing required parameter path!");

    let data: Vec<String> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|l| l.expect("Could not read line!"))
        .filter(|l| l != "")
        .flat_map(|s| s.split(',').map(str::to_owned).collect::<Vec<_>>())
        .collect();

    println!(
        "Sum of steps: {}",
        data.iter().map(|s| hash(s.as_str()) as usize).sum::<usize>()
    );

    let mut lense_array = LenseArray { boxes: vec![vec![]; 256].try_into().unwrap() };
    for operation in data.iter() {
        if operation.contains('=') {
            // add lense
            let (label, fl) = operation.split_once('=').expect("Invalid operation!");
            let lense = Lense {
                label: String::from(label),
                focal_lenght: fl.parse::<usize>().expect("Invalid focal lenght!")
            };
            lense_array.add_lense(lense);
        } else {
            // remove lense
            let label = String::from(operation.trim_end_matches('-'));
            lense_array.remove_lense(label);
        }
    }

    println!(
        "Power: {}",
        lense_array.get_power()
    )
    
}


#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test_hash_func() {
        assert_eq!(hash("HASH"), 52);
    }

}
