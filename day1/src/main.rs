use std::fs;

fn main() {
                                                            // error message if unable to read to string
    let content = fs::read_to_string("input").expect("Unable to read file");

    let counts = content
        // split converts it into an iterator
        .split("\n\n")
        // similar to a for loop, acts on "chunk" and returns a usize         // convert string to int and sum it
        .map(|chunk | -> usize { chunk.split("\n").map(|calories| calories.parse().unwrap_or(0)).sum()});
    
    // collect counts as a vector. <_> can automatically identify what type is being returned
    let mut v:Vec<_> = counts.collect();
    v.sort();
    let largest_num = v.len() - 1;
    println!("Largest calories is {}\n", v[largest_num]);
}
