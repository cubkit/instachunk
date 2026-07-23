use instachunk;

#[test]
fn main() {
    let mut colours: Vec<[u8; 3]> = vec![];
    for biome in 0..instachunk::biomes::MAXBIOMES {
        let biomec = instachunk::biomes::biomecolour(biome);
        println!("Checking biome {} colour", biome);
        for colour in &colours {
            assert!(biomec != *colour);
        }
        colours.push(biomec);
    }
}
