use instachunk;

#[test]
fn main() {
    let mut colours: Vec<[u8; 3]> = vec![];
    for biomen in 0..u8::MAX {
        if let Some(biome) = instachunk::biomes::Biome::from_u8(biomen) {
            let biomec = biome.colour();
            println!("Checking biome {:?} ({}) colour", biome, biome as u8);
            assert!(!colours.contains(&biomec));
            colours.push(biomec);
        } else {
            // End of biomes
            break;
        }
    }
}
