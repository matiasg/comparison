use std::io;
use std::collections::HashSet;

struct AssociativitySequence {
    seq: Vec<usize>,
}

fn main() {
    println!("Number to start with: ");
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("wtf");
    let size :usize = size.trim().parse().expect("just write a number!");

    let mut numbers :Vec<i32> = (1..=(size as i32)).collect();
    numbers.reverse();

    let mut results = HashSet::new();

    for asseq in &catalan(size) {
        for opps in ops_sequence(size) {
            let result = apply(&opps, &asseq, &numbers);
            if result > 0 {
                if results.insert(result) {
                    //println!("got a new one: {}. Total: {}", result, results.len());
                }
            }
        }

    }
    println!("There are {} different positive results", results.len());
    //let mut results: Vec<i32> = results.into_iter().collect();
    //results.sort();
    //println!("All results: {:?}", results);
}

fn catalan(size: usize) -> Vec<AssociativitySequence> {
    if size == 1 {
        return vec![AssociativitySequence{seq: vec![]}];
    }
    if size == 2 {
        return vec![AssociativitySequence{seq: vec![1]}];
    }

    let mut ret: Vec<AssociativitySequence> = Vec::new();
    for b in 1..size {
        for c1 in catalan(b) {
            for c2 in catalan(size - b) {
                let new_seq = [
                    c1.seq.clone(),
                    c2.seq.into_iter().map(|j| {j + 1}).collect(),
                    vec![1]
                ].concat();
                ret.push(AssociativitySequence{seq: new_seq});
            }
        }
    }
    return  ret;
}

fn addition(a: i32, b: i32) -> i32 { a + b }
fn subtraction(a: i32, b: i32) -> i32 { a - b }
fn product(a: i32, b: i32) -> i32 { a * b }

static ALL_OPERATORS: [fn(i32, i32) -> i32; 3] = [addition, subtraction, product];

struct Opperations {
    ops: Vec<fn(i32, i32) -> i32>,

}

fn add_at_end(prev: &Opperations, cur: fn(i32, i32) -> i32) -> Opperations {
    Opperations{ops: [prev.ops.clone(), vec![cur]].concat()}
}

fn ops_sequence(size: usize) -> Vec<Opperations> {
    if size == 1 {
        return vec![Opperations{ops: vec![]}];
    }
    let mut ret: Vec<Opperations> = Vec::new();
    for prev in ops_sequence(size - 1) {
        for cur in ALL_OPERATORS {
            ret.push(add_at_end(&prev, cur));
        }
    }
    return ret;
}

fn apply(opps: &Opperations, asseq: &AssociativitySequence, numbers: &Vec<i32>) -> i32 {
    assert_eq!(opps.ops.len(), asseq.seq.len());
    assert_eq!(numbers.len(), asseq.seq.len() + 1);
    let mut current_numbers = numbers.clone();
    for i in 0..asseq.seq.len() {
        let pos = asseq.seq[i];
        // apply the operator to two consecutive numbers
        let result = opps.ops[i](current_numbers[pos - 1], current_numbers[pos]);
        current_numbers = [
            current_numbers[0..pos-1].to_vec(),
            vec![result],
            current_numbers[pos+1..].to_vec()
        ].concat();
    }
    current_numbers[0]
}
