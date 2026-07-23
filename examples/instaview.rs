use instachunk::makechunk;

use std::env;
use std::fs;
use std::path;

use image::{RgbImage, ImageBuffer};

fn main() {
    let args: Vec<String> = env::args().collect(); // seed chunk_size chunk_x chunk_y brush_size height_max frequency
    let seed: i64 = if args.len() > 1 as usize {
        args[1].parse().unwrap()
    } else {
        1
    }; // Will be changed to a input (is i64 a 64-bit integer?? (as i32 is also valid) (oh, it's signed))

    let chunk_x: i16 = if args.len() > 2 as usize {
        args[2].parse().unwrap()
    } else {
        0
    }; // and another..

    let chunk_y: i16 = if args.len() > 3 as usize {
        args[3].parse().unwrap()
    } else {
        0
    }; // yet another input

    let scale: u8 = if args.len() > 4 as usize {
        args[4].parse().unwrap()
    } else {
        1
    };

    let w: u8 = if args.len() > 5 as usize {
        args[5].parse().unwrap()
    } else {
        1
    };

    let h: u8 = if args.len() > 6 as usize {
        args[6].parse().unwrap()
    } else {
        1
    };

    println!("Hello, world!");
    // make output folder
    if !(path::Path::new("out").exists()) {
        let _ = fs::create_dir(path::Path::new("out"));
    }
    mapchunksquare(seed, chunk_x, chunk_y, w, h, scale);

}

fn mapchunksquare(seed: i64, chunk_x: i16, chunk_y: i16, w: u8, h: u8, scale: u8) {
    // make images
    let width = (instachunk::CHUNKSIZE as u32)*(w as u32);
    let height = (instachunk::CHUNKSIZE as u32)*(h as u32);
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

    for x in 0..(w as i16) {
        for y in 0..(h as i16) {
            let chunk = makechunk(seed, chunk_x+x, chunk_y+y, scale);
            let rx = (x*(instachunk::CHUNKSIZE as i16)) as u32;
            let ry = (y*(instachunk::CHUNKSIZE as i16)) as u32;
            paste(&mut biomemap, &outbiomemap(&chunk.biomemap), rx, ry);
            paste(&mut treemap, &outboolmap(&chunk.treemap), rx, ry);

            paste(&mut heightmap, &outmap(&chunk.heightmap), rx, ry);
            paste(&mut heatmap, &outmap(&chunk.heatmap), rx, ry);
            paste(&mut follagemap, &outmap(&chunk.follagemap), rx, ry);
            paste(&mut moisturemap, &outmap(&chunk.moisturemap), rx, ry);
            paste(&mut rivermap, &outmap(&chunk.rivermap), rx, ry);
            paste(&mut oddmap, &outmap(&chunk.oddmap), rx, ry);
        }
    }

    // make folder
    let path = format!("out/full {} {} - {} {} ({})", chunk_x, chunk_y, chunk_x+(w as i16), chunk_y+(h as i16), scale);
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

fn outmap(map: &Vec<Vec<u8>>) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let h = map[x as usize][y as usize];
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb([h, h ,h]);
        }
    }
    return imagemap;
}

fn outbiomemap(map: &Vec<Vec<u8>>) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let colour = instachunk::biomes::biomecolour(map[x as usize][y as usize]);
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb(colour);
        }
    }
    return imagemap;
}

fn outboolmap(map: &Vec<Vec<bool>>) -> RgbImage {
    let mut imagemap: RgbImage = ImageBuffer::new(instachunk::CHUNKSIZE as u32, instachunk::CHUNKSIZE as u32);
    for x in 0..instachunk::CHUNKSIZE {
        for y in 0..instachunk::CHUNKSIZE {
            let c =  if map[x as usize][y as usize] {255} else {0};
            *imagemap.get_pixel_mut(x as u32, y as u32) = image::Rgb([c, c, c]);
        }
    }
    return imagemap;
}