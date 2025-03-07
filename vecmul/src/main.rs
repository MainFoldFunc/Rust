use std::io;

fn main() {
    let vec1 = addvector();
    let vec2 = addvector();

    if vec1.len() != vec2.len() {
        println!("Error: The vectors must have the same length.");
        return;
    }

    let vecres = mul(&vec1, &vec2);

    println!("The result of multiplying these vectors: {:?}", vecres);
}

fn addvector() -> Vec<i32> {
    let mut input = String::new();
    println!("Enter a vector (separate numbers with a comma):");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");

    input
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .filter_map(Result::ok)
        .collect()
}

fn mul(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    let mut vecres = Vec::new();

    for (element1, element2) in vec1.iter().zip(vec2.iter()) {
        vecres.push(element1 * element2);
    }

    vecres
}
