use instachunk::{makechunk, Chunk, biomes, CHUNKSIZE};

use std::env;
use std::fs;
use std::path;
use std::thread;

use image::{RgbImage, ImageBuffer};

fn main() {
    let args: Vec<String> = env::args().collect(); // seed, chunk_x, chunk_y, scale, view width, view height
    let seed: i64 = if args.len() > 1 as usize {
        args[1].parse().unwrap()
    } else {
        1
    }; // Will be changed to a input (is i64 a 64-bit integer?? (as i32 is also valid) (oh, it's signed))

    let chunk_x: i32 = if args.len() > 2 as usize {
        args[2].parse().unwrap()
    } else {
        0
    }; // and another..

    let chunk_y: i32 = if args.len() > 3 as usize {
        args[3].parse().unwrap()
    } else {
        0
    }; // yet another input

    let scale: usize = if args.len() > 4 as usize {
        args[4].parse().unwrap()
    } else {
        1
    };

    let w: u32 = if args.len() > 5 as usize {
        args[5].parse().unwrap()
    } else {
        1
    };

    let h: u32 = if args.len() > 6 as usize {
        args[6].parse().unwrap()
    } else {
        1
    };

    let do_threads = args.contains(&"--threaded".to_string());
    let ignore_ice = args.contains(&"--no-ice".to_string());

    println!("Hello, world!");
    // make output folder
    if !(path::Path::new("out").exists()) {
        let _ = fs::create_dir(path::Path::new("out"));
    }
    mapchunksquare(seed, chunk_x, chunk_y, w, h, scale, do_threads, ignore_ice);

}

fn mapchunksquare(seed: i64, chunk_x: i32, chunk_y: i32, width: u32, height: u32, scale: usize, threaded: bool, ignore_ice: bool) {
    let (uwidth, uheight) = (width, height);
    let (iwidth, iheight) = (width as i32, height as i32);
    // make images
    let width = (instachunk::CHUNKSIZE as u32)*width;
    let height = (instachunk::CHUNKSIZE as u32)*height;
    // special maps
    let mut biomemap: RgbImage = ImageBuffer::new(width, height);
    let mut treemap: RgbImage = ImageBuffer::new(width, height);
    // normal maps
    let mut heightmap: RgbImage = ImageBuffer::new(width, height);
    let mut heatmap: RgbImage = ImageBuffer::new(width, height);
    let mut follagemap: RgbImage = ImageBuffer::new(width, height);
    let mut moisturemap: RgbImage = ImageBuffer::new(width, height);
    let mut rivermap: RgbImage = ImageBuffer::new(width, height);
    let mut oddmap: RgbImage = ImageBuffer::new(width, height);

    let amount = uwidth * uheight;

    println!("Creating chunks...");
    let chunk_list: Vec<Chunk>;
    if threaded {
        let mut children = vec![]; // All the chunks
        // Create chunks
        for x in 0..(iwidth) {
            for y in 0..(iheight) {
                {
                    let n = ((x * iheight) + y) as usize;
                    let progress = (n * 32) / (amount as usize);
                    println!("\x1B[FCreating chunks [{:<32}]   ", "=".repeat(progress)+">");
                }
                children.push(thread::spawn(move || -> Chunk {
                    makechunk(seed, chunk_x+x, chunk_y+y, scale)
                }));
            }
        }
        println!("Merging...                                          \n");
        chunk_list = children.into_iter().map(|c| c.join().unwrap()).collect(); // All the chunks
    } else {
        let mut chunks = vec![];
        // Create chunks
        for x in 0..(iwidth) {
            for y in 0..(iheight) {
                {
                    let n = ((x * iheight) + y) as usize;
                    let progress = (n * 32) / (amount as usize);
                    println!("\x1B[FCreating chunks [{:<32}]   ", "=".repeat(progress)+">");
                }
                chunks.push(makechunk(seed, chunk_x+x, chunk_y+y, scale));
            }
        }
        chunk_list = chunks;
    }

    // Draw chunks
    for x in 0..(iwidth) {
        for y in 0..(iheight) {
            let n = ((x * iheight) + y) as usize;
            let chunk = &chunk_list[n];
            {
                let progress = (n * 32) / (amount as usize);
                println!("\x1B[FDrawing chunks [{:<32}]   ", "=".repeat(progress)+">");
            }
            let rx = (x*(instachunk::CHUNKSIZE as i32)) as u32;
            let ry = (y*(instachunk::CHUNKSIZE as i32)) as u32;
            paste(&mut biomemap, &outbiomemap(&chunk.biomemap, ignore_ice), rx, ry);
            paste(&mut treemap, &outboolmap(&chunk.treemap), rx, ry);

            paste(&mut heightmap, &outmap(&chunk.heightmap), rx, ry);
            paste(&mut heatmap, &outmap(&chunk.heatmap), rx, ry);
            paste(&mut follagemap, &outmap(&chunk.follagemap), rx, ry);
            paste(&mut moisturemap, &outmap(&chunk.moisturemap), rx, ry);
            paste(&mut rivermap, &outmap(&chunk.rivermap), rx, ry);
            paste(&mut oddmap, &outmap(&chunk.oddmap), rx, ry);
        }
    }

    println!("Saving...");

    // make folder
    let path = format!("out/full {} {} - {} {} ({})", chunk_x, chunk_y, chunk_x+iwidth, chunk_y+iheight, scale);
    if !(path::Path::new(&path).exists()) {
        let _ = fs::create_dir(path::Path::new(&path));
    }
    // save images
    biomemap.save(format!("{}/biomemap.png", path)).unwrap();
    treemap.save(format!("{}/treemap.png", path)).unwrap();
    heightmap.save(format!("{}/heightmap.png", path)).unwrap();
    heatmap.save(format!("{}/heatmap.png", path)).unwrap();
    follagemap.save(format!("{}/follagemap.png", path)).unwrap();
    moisturemap.save(format!("{}/moisturemap.png", path)).unwrap();
    rivermap.save(format!("{}/rivermap.png", path)).unwrap();
    oddmap.save(format!("{}/oddmap.png", path)).unwrap();
}

fn paste(img: &mut RgbImage, img2: &RgbImage, x: u32, y:u32) {
    let paste_x = x;
    let paste_y = y;
    for x in 0..img2.width() {
        for y in 0..img2.height() {
            *img.get_pixel_mut(paste_x+x, paste_y+y) = *img2.get_pixel(x, y)
        }
    }
}

fn outmap(map: &[[u8; CHUNKSIZE]; CHUNKSIZE]) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let h = map[x as usize][y as usize];
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb([h, h ,h]);
        }
    }
    return imagemap;
}

#[allow(dead_code)]
fn outmap_u16(map: &[[u16; CHUNKSIZE]; CHUNKSIZE]) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let h = map[x as usize][y as usize];
            let low = (h >> 8) as u8;
            let high = (h & 0xff) as u8;
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb([low, high, 0]);
        }
    }
    return imagemap;
}

fn outbiomemap(map: &[[biomes::Biome; CHUNKSIZE]; CHUNKSIZE], ignore_ice: bool) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let biome = match map[x as usize][y as usize] {
                instachunk::biomes::FrozenOcean => {
                    if ignore_ice {
                        instachunk::biomes::Ocean
                    } else {
                        instachunk::biomes::FrozenOcean
                    }
                }
                instachunk::biomes::FrozenShore => {
                    if ignore_ice {
                        instachunk::biomes::Shore
                    } else {
                        instachunk::biomes::FrozenShore
                    }
                }
                b => b
            };
            let colour = biome.colour();
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb(colour);
        }
    }
    return imagemap;
}

fn outboolmap(map: &[[bool; CHUNKSIZE]; CHUNKSIZE]) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let c =  if map[x as usize][y as usize] {255} else {0};
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb([c, c, c]);
        }
    }
    return imagemap;
}
