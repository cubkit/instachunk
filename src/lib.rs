use opensimplex_noise_rs::OpenSimplexNoise; // Unlicense!
use rand_seeder::Seeder;
use rand::{RngExt, rngs::Xoshiro256PlusPlus};
pub mod biomes;

pub const HEIGHTMAX: u8    = 255;
// pub const BIG_HEIGHTMAX: u16 = 20_000;
pub const CHUNKSIZE: usize = 48;

/// A chunk
///
/// Contains all the assosiated maps
#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub heightmap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub heatmap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub follagemap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub moisturemap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub rivermap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub oddmap: [[u8; CHUNKSIZE]; CHUNKSIZE],
    pub biomemap: [[biomes::Biome; CHUNKSIZE]; CHUNKSIZE],
    pub treemap: [[bool; CHUNKSIZE]; CHUNKSIZE]
}

/// Make a 48 x 48 chunk
pub fn makechunk(seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> Chunk {
    // make basemaps
    let mut heightmap = make_heightmap(seed, chunk_x, chunk_y, scale);
    let heatmap       = make_heatmap(seed+1, chunk_x, chunk_y, scale);
    let follagemap    = make_noisemap(seed+2, chunk_x, chunk_y, scale);
    let moisturemap   = make_noisemap(seed+3, chunk_x, chunk_y, scale);
    let rivermap      = make_rivermap(seed+4, chunk_x, chunk_y, scale);
    let oddmap        = make_noisemap(seed+5, chunk_x, chunk_y, scale);

    // re-edit the heightmap
    edit_heightmap(&make_biomemap(&heightmap, &heatmap, &follagemap, &moisturemap, None, &oddmap), &mut heightmap);

    // make complex maps
    let biomemap = make_biomemap(&heightmap, &heatmap, &follagemap, &moisturemap, Some(&rivermap), &oddmap);
    let treemap  = make_treemap(&biomemap, seed+6, chunk_x, chunk_y, scale);

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

/// Edit the height according to the biome
///
/// Ex. make terraces in a badlands
pub fn edit_heightmap(biomemap: &[[biomes::Biome; CHUNKSIZE]; CHUNKSIZE], heightmap: &mut [[u8; CHUNKSIZE]; CHUNKSIZE]) {
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            heightmap[x][y] = biomemap[x][y].edit_height(heightmap[x][y]);
        }
    }
}

/// Make a map full of positions for trees
pub fn make_treemap(biomemap: &[[biomes::Biome; CHUNKSIZE]; CHUNKSIZE], seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> [[bool; CHUNKSIZE]; CHUNKSIZE] {
    let mut map = [[false; CHUNKSIZE]; CHUNKSIZE];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            let biome = &biomemap[x][y];
            let id = format!("{} {} {}", (x as i32)+(chunk_x*(CHUNKSIZE as i32)), (y as i32)+(chunk_y*(CHUNKSIZE as i32)), seed);
            let mut rng: Xoshiro256PlusPlus = Seeder::from(id).into_rng();
            if !(biome.is_treeless()) {
                if (biome.is_forested() && rng.random::<f64>() < 0.5) || (biome.is_semi_forested() && rng.random::<f64>() < 0.25) || (!(biome.is_forested() || biome.is_semi_forested()) && rng.random::<f64>() < 0.01){
                    if ((x%scale == 0) && (y%scale == 0)) || true {
                        map[x][y] = true;
                    }
                }
            }
        }
    }
    return map;
}

/// Make a map of biomes from a set of maps (heightmap, heatmap, follagemap, ...)
pub fn make_biomemap(heightmap: &[[u8; CHUNKSIZE]; CHUNKSIZE], heatmap: &[[u8; CHUNKSIZE]; CHUNKSIZE], follagemap: &[[u8; CHUNKSIZE]; CHUNKSIZE], moisturemap: &[[u8; CHUNKSIZE]; CHUNKSIZE], rivermap: Option<&[[u8; CHUNKSIZE]; CHUNKSIZE]>, oddmap: &[[u8; CHUNKSIZE]; CHUNKSIZE]) -> [[biomes::Biome; CHUNKSIZE]; CHUNKSIZE] {
    let mut map = [[biomes::Blank; CHUNKSIZE]; CHUNKSIZE];
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            map[x][y] = biomes::biome(
                heightmap[x][y],
                heatmap[x][y],
                follagemap[x][y],
                moisturemap[x][y],
                match rivermap {Some(rmap) => rmap[x][y], None => 0},
                oddmap[x][y]
            )
        }
    }
    return map;
}

/// Make a heat map for a 48 x 48 chunk
pub fn make_heatmap(seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> [[u8; CHUNKSIZE]; CHUNKSIZE] {
    let mut map = [[0.0; CHUNKSIZE]; CHUNKSIZE];
    generate(&mut map, seed, chunk_x, chunk_y, 255, 0.8,   scale * 2);
    generate(&mut map, seed, chunk_x, chunk_y,  16, 0.15,  scale);
    generate(&mut map, seed, chunk_x, chunk_y,   8, 0.05,  scale);
    // max(&mut map, HEIGHTMAX); // Did not do anything
    return convert_f64_to_u8_grid(map);
}

/// Make a height map for a 48 x 48 chunk
pub fn make_heightmap(seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> [[u8; CHUNKSIZE]; CHUNKSIZE] {
    let mut fmap = [[0.0; CHUNKSIZE]; CHUNKSIZE];
    generate(&mut fmap, seed, chunk_x, chunk_y, 255, 0.6,  scale);
    generate(&mut fmap, seed, chunk_x, chunk_y, 128, 0.2,  scale);
    generate(&mut fmap, seed, chunk_x, chunk_y,  64, 0.1,  scale);
    generate(&mut fmap, seed, chunk_x, chunk_y,   8, 0.075, scale);
    generate(&mut fmap, seed, chunk_x, chunk_y,   4, 0.025,  scale);
    // Load into grid of u8 (from f64 (0.0 - 1.0))
    let mut map = [[0; CHUNKSIZE]; CHUNKSIZE];
    {
        let mut emap = [[0.0; CHUNKSIZE]; CHUNKSIZE];
        generate(&mut emap, seed, chunk_x, chunk_y, 255, 2.5,  scale);
        let height_max = HEIGHTMAX as f64;
        for x in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                let height = fmap[x][y];
                map[x][y] = (height.powf(emap[x][y]) * height_max) as u8;
            }
        }
    }
    return map;
}

/// Make a general noise map for a 48 x 48 chunk
pub fn make_noisemap(seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> [[u8; CHUNKSIZE]; CHUNKSIZE] {
    let mut map = [[0.0; CHUNKSIZE]; CHUNKSIZE];
    generate(&mut map, seed, chunk_x, chunk_y, 128, 0.6, scale);
    generate(&mut map, seed, chunk_x, chunk_y,  16, 0.2, scale);
    generate(&mut map, seed, chunk_x, chunk_y,   8, 0.1, scale);
    generate(&mut map, seed, chunk_x, chunk_y,   4, 0.1, scale);
    // max(&mut map, HEIGHTMAX); // Did not do anything
    return convert_f64_to_u8_grid(map);
}

/// Make a noise map for rivers for a 48 x 48 chunk
pub fn make_rivermap(seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) -> [[u8; CHUNKSIZE]; CHUNKSIZE] {
    let mut map = [[0.0; CHUNKSIZE]; CHUNKSIZE];
    generate(&mut map, seed, chunk_x, chunk_y, 128, 0.6, scale);
    generate(&mut map, seed, chunk_x, chunk_y,  16, 0.3, scale);
    generate(&mut map, seed, chunk_x, chunk_y,   8, 0.1, scale);
    return convert_f64_to_u8_grid(map);
}

/// Generate a 48 x 48 chunk of noise
fn generate(map: &mut [[f64; CHUNKSIZE]; CHUNKSIZE], seed: i64, chunk_x: i32, chunk_y: i32, size: u8, weight: f64, scale: usize) {
    let ichunk_size = CHUNKSIZE as i32;
    // assert!(0.0 <= weight && weight <= 1.0);
    let size = (size as f64) * (scale as f64);
    let noise_gen = OpenSimplexNoise::new(Some(seed));
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            let mut height = noise_gen.eval_2d((((chunk_x*ichunk_size)+(x as i32)) as f64) / size, (((chunk_y*ichunk_size)+(y as i32)) as f64) / size);
            height = (height + 1.0) / 2.0;
            map[x][y] += height * weight;
        }
    }
}

/// Convert a grid of f64 (0.0 - 1.0) to a grid of u8  (0 - 255)
fn convert_f64_to_u8_grid(map: [[f64; CHUNKSIZE]; CHUNKSIZE]) -> [[u8; CHUNKSIZE]; CHUNKSIZE] {
    let mut out = [[0; CHUNKSIZE]; CHUNKSIZE];
    let height_max = HEIGHTMAX as f64;
    for x in 0..CHUNKSIZE {
        for y in 0..CHUNKSIZE {
            out[x][y] = (map[x][y] * height_max) as u8;
        }
    }
    return out;
}

// TODO: Sharper mountains
// TODO: Flatter ground
