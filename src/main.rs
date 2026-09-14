use std::env;
use instachunk;

fn main() {
    println!("Hello, world!");
    println!("Instachunk started...");
    let args: Vec<String> = env::args().collect(); // seed chunk_x chunk_y scale
    let seed: i64 = args[1].parse().unwrap(); // Input !!!
    let chunk_x: i32 = args[2].parse().unwrap(); // and another..
    let chunk_y: i32 = args[3].parse().unwrap(); // yet another input
    let scale: usize = if args.len() > 4 as usize { // Optional Input!
        args[4].parse().unwrap()
    } else {
        1
    };

    let chunk = instachunk::makechunk(seed, chunk_x, chunk_y, scale);

    // output
    println!(
        "maps (heightmap, heatmap, follagemap, moisturemap, rivermap, oddmap, treemap, biomemap): |{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|",
        chunk.heightmap,
        chunk.heatmap,
        chunk.follagemap,
        chunk.moisturemap,
        chunk.rivermap,
        chunk.oddmap,
        chunk.treemap,
        chunk.biomemap.into_iter().map(
            |l| {
                l.into_iter().map(
                    |v| {
                        v as u8
                    }
                ).collect::<Vec<u8>>()
            }
        ).collect::<Vec<Vec<u8>>>()
    );
}
