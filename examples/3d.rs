use instachunk::{makechunk, biomes, CHUNKSIZE, HEIGHTMAX};
use image::{RgbImage, ImageBuffer, Rgb};
use std::path;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // seed, chunk_x, chunk_y, scale, view width, view height
    let seed: i64 = if args.len() > 1 {args[1].parse().unwrap()} else {1}; // The seed of the world
    let chunk_x: i32 = if args.len() > 2 { args[2].parse().unwrap()} else {0}; // The x of the chunk
    let chunk_y: i32 = if args.len() > 3 { args[3].parse().unwrap()} else {0}; // The y of the chunk
    let scale: usize = if args.len() > 4 { args[4].parse().unwrap()} else {1}; // The scale of the world
    let chunks_w: u32 = if args.len() > 5 { args[5].parse().unwrap()} else {1}; // The x of the chunk
    let chunks_h: u32 = if args.len() > 6 { args[6].parse().unwrap()} else {1}; // The y of the chunk
    // make output folder
    if !(path::Path::new("out-3d").exists()) {
        let _ = fs::create_dir(path::Path::new("out-3d"));
    }
    // Create image
    println!("Creating Image...");
    let visualization = mapchunksquare(seed, scale, chunk_x, chunk_y, chunks_w, chunks_h);
    // Save image
    println!("Saving...");
    visualization.save(format!("out-3d/{}, {}; {}x{} ({} {}:1).png", chunk_x, chunk_y, chunks_w, chunks_h, seed, scale)).unwrap();
}


fn mapchunksquare(seed: i64, scale: usize, chunk_x: i32, chunk_y: i32, chunks_w: u32, chunks_h: u32) -> RgbImage {
    // Total chunk size
    let chunk_size = CHUNKSIZE as u32;
    let twidth = ((chunks_w - 1) * chunk_size) / 2 + chunk_size;
    let theight = chunk_size * chunks_h;
    // Create image
    let offx = theight - chunk_size;
    let offy = HEIGHTMAX as u32;
    let im_width = (twidth * 2) + offx;
    let im_height = theight + (twidth / 2) + offy;
    let mut visualization: RgbImage = ImageBuffer::new(im_width, im_height);
    for x in 0..chunks_w {
        for y in 0..chunks_h {
            draw_chunk(&mut visualization, x, y, offx, offy, seed, chunk_x + (x as i32), chunk_y + (y as i32), scale);
        }
    }
    return visualization;
}

fn draw_chunk(image: &mut RgbImage, cx: u32, cy: u32, offx: u32, offy: u32, seed: i64, chunk_x: i32, chunk_y: i32, scale: usize) {
    // Total chunk size
    let twidth = CHUNKSIZE as u32;
    let theight = CHUNKSIZE as u32;
    // Generate chunk
    let biomemap;
    let heightmap;
    {
        let chunk = makechunk(seed, chunk_x, chunk_y, scale);
        biomemap = chunk.biomemap;
        heightmap = chunk.heightmap;
    }
    // Draw
    let (cx, cy) = (cx * twidth, cy * theight);
    let (im_width, im_height) = (image.width(), image.height());
    for x in 0..twidth {
        for y in 0..theight {
            // Get the biome & the height
            let biome = biomemap[x as usize][y as usize];
            let height = heightmap[x as usize][y as usize].max(biomes::BEACHHEIGHT) as u32;
            // Calculate position to draw to
            let dx: u32 = offx + (x + cx) + twidth - (y + cy); // (((cx - 1) * twidth) / 2)
            let dy: u32 = offy + (((y + cy) + (x + cx)) / 2);
            // Draw the spot
            let colour = biome.colour();
            let fheight = (height as f32) / (HEIGHTMAX as f32);
            let shaded_colour = [((colour[0] as f32) * fheight) as u8, ((colour[1] as f32) * fheight) as u8, ((colour[2] as f32) * fheight) as u8];
            for oy in 0..height {
                if dx < im_width && (dy - oy) < im_height {
                    *image.get_pixel_mut(dx, dy - oy) = Rgb(shaded_colour);
                }
            }
        }
    }
}
