use std::env;
use instachunk;

fn main() {
    println!("Hello, world!");
    println!("Instachunk started...");
    let args: Vec<String> = env::args().collect(); // seed chunk_size chunk_x chunk_y brush_size height_max frequency
    let seed: i64 = args[1].parse().unwrap(); // Input !!!
    let chunk_x: i16 = args[2].parse().unwrap(); // and another..
    let chunk_y: i16 = args[3].parse().unwrap(); // yet another input
    let scale: u8 = if args.len() > 4 as usize { // Optional Input!
        args[4].parse().unwrap()
    } else {
        1
    };

    let chunk = instachunk::makechunk(seed, chunk_x, chunk_y, scale);

    // output
    println!(
        "maps (heightmap, heatmap, follagemap, moisturemap, rivermap, oddmap, treemap, biomemap): |{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|",
        chunk.heightmap, chunk.heatmap, chunk.follagemap, chunk.moisturemap, chunk.rivermap, chunk.oddmap, chunk.treemap, chunk.biomemap
    );
}