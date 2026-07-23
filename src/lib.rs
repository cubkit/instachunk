use opensimplex_noise_rs::OpenSimplexNoise; // Unlicense!
use rand_seeder::Seeder;
use rand::{RngExt, rngs::Xoshiro256PlusPlus};
pub mod biomes;

pub static HEIGHTMAX: u8 = 255;
pub static CHUNKSIZE: u8 = 48;

pub struct Chunk {
    pub heightmap: Vec<Vec<u8>>,
    pub heatmap: Vec<Vec<u8>>,
    pub follagemap: Vec<Vec<u8>>,
    pub moisturemap: Vec<Vec<u8>>,
    pub rivermap: Vec<Vec<u8>>,
    pub oddmap: Vec<Vec<u8>>,
    pub biomemap: Vec<Vec<u8>>,
    pub treemap: Vec<Vec<bool>>
}

pub fn makechunk(seed: i64, chunk_x: i16, chunk_y: i16, scale: u8) -> Chunk {
    // calc important vars
    //println!("seed: {}", seed);
    //println!("chunk: {} {}", chunk_x, chunk_y);

    // make basemaps
    let heightmap = makeheightmap(seed, chunk_x, chunk_y, scale);
    let heatmap = makeheatmap(seed+1, chunk_x, chunk_y, scale);
    let follagemap = makeheightmap(seed+2, chunk_x, chunk_y, scale);
    let moisturemap = makeheightmap(seed+3, chunk_x, chunk_y, scale);
    let rivermap = makeheightmap(seed+4, chunk_x, chunk_y, scale);
    let oddmap = makeheightmap(seed+5, chunk_x, chunk_y, scale);

    // make complex maps
    let biomemap = makebiomemap(&heightmap, &heatmap, &follagemap, &moisturemap, &rivermap, &oddmap);
    let treemap = maketreemap(&biomemap, seed+6, chunk_x, chunk_y, scale);
    return Chunk {
        heightmap,
        heatmap,
        follagemap,
        moisturemap,
        rivermap,
        oddmap,
        biomemap,
        treemap
    };
}

pub fn maketreemap(biomemap: &Vec<Vec<u8>>, seed: i64, chunk_x: i16, chunk_y: i16, scale: u8) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = vec![vec![false; CHUNKSIZE as usize]; CHUNKSIZE as usize];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            let biome = biomemap[x as usize][y as usize];
            let id = format!("{} {} {}", (x as i16)+(chunk_x*(CHUNKSIZE as i16)), (y as i16)+(chunk_y*(CHUNKSIZE as i16)), seed);
            let mut rng: Xoshiro256PlusPlus = Seeder::from(id).into_rng();
            if !(biomes::istreeless(biome)) {
                if (biomes::isforest(biome) && rng.random::<f64>() < 0.5) || (biomes::issemiforest(biome) && rng.random::<f64>() < 0.25) || (!(biomes::isforest(biome) || biomes::issemiforest(biome)) && rng.random::<f64>() < 0.01){
                    if ((x%scale == 0) && (y%scale == 0)) || true {
                        map[x as usize][y as usize] = true;
                    }
                }
            }
        }
    }
    return map;
}

pub fn makebiomemap(heightmap: &Vec<Vec<u8>>, heatmap: &Vec<Vec<u8>>, follagemap: &Vec<Vec<u8>>, moisturemap: &Vec<Vec<u8>>, rivermap: &Vec<Vec<u8>>, oddmap: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![vec![biomes::BLANK; CHUNKSIZE as usize]; CHUNKSIZE as usize];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            map[x as usize][y as usize] = biomes::biome(
                HEIGHTMAX,
                heightmap[x as usize][y as usize],
                heatmap[x as usize][y as usize],
                follagemap[x as usize][y as usize],
                moisturemap[x as usize][y as usize],
                rivermap[x as usize][y as usize],
                oddmap[x as usize][y as usize]
            )
        }
    }
    return map;
}

pub fn makeheatmap(seed: i64, chunk_x: i16, chunk_y: i16, scale: u8) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![vec![0; CHUNKSIZE as usize]; CHUNKSIZE as usize];
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 255, 0.6, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 64, 0.2, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 32, 0.1, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 8, 0.1, scale);
    max(&mut map);
    return map;
}

pub fn makeheightmap(seed: i64, chunk_x: i16, chunk_y: i16, scale: u8) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![vec![0; CHUNKSIZE as usize]; CHUNKSIZE as usize];
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 128, 0.6, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 16, 0.2, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 8, 0.1, scale);
    generate(&mut map, seed, chunk_x, chunk_y, HEIGHTMAX, CHUNKSIZE, 4, 0.1, scale);
    max(&mut map);
    return map;
}

pub fn generate(map: &mut Vec<Vec<u8>>, seed: i64, chunk_x: i16, chunk_y: i16, height_max: u8, chunk_size: u8, size:u8, weight:f64, scale: u8) {
    let chunk_size= chunk_size as i16;
    assert!(0.0 <= weight && weight <= 1.0);
    let size = (size as f64)*(scale as f64);
    let noise_gen = OpenSimplexNoise::new(Some(seed));
    for x in 0..chunk_size {
        for y in 0..chunk_size {
            let mut height = noise_gen.eval_2d((((chunk_x*chunk_size)+x) as f64)/size, (((chunk_y*chunk_size)+y) as f64)/size);
            height = (height + 1.0)/2.0;
            map[x as usize][y as usize] = map[x as usize][y as usize]+(((height*weight)*(height_max as f64)) as u8);
        }
    }
}

pub fn max(map: &mut Vec<Vec<u8>>) {
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            if map[x as usize][y as usize] > HEIGHTMAX {
                map[x as usize][y as usize] = HEIGHTMAX;
            }
        }
    }
}