#![allow(non_snake_case)]
use gif::{Encoder, Frame, Repeat};
use rand::Rng;
use std::fs::File;

fn update(
    popn: &Vec<Vec<u8>>,
    HEIGHT: usize,
    WIDTH: usize,
    x: i32,
    y: i32,
    next_value: u8,
) -> bool {
    let mut count: u8 = 0;
    // CIRCULAR COORDIANTES FILTER
    let mut filter: [(i32, i32); 8] = [
        (0 + x, 1 + y),
        (1 + x, 0 + y),
        (0 + x, -1 + y),
        (-1 + x, 0 + y),
        (1 + x, 1 + y),
        (-1 + x, -1 + y),
        (1 + x, -1 + y),
        (-1 + x, 1 + y),
    ];
    for i in 0..filter.len() {
        // filter[i].0 += x as i32;
        // filter[i].1 += y as i32;
        if filter[i].1 > HEIGHT as i32 {
            filter[i].1 = HEIGHT as i32 - filter[i].1;
        }
        if filter[i].1 < 0 {
            filter[i].1 = HEIGHT as i32 + filter[i].1;
        }
        if filter[i].0 > WIDTH as i32 {
            filter[i].0 = WIDTH as i32 - filter[i].0;
        }
        if filter[i].0 < 0 {
            filter[i].0 = WIDTH as i32 + filter[i].0;
        }
        if popn[filter[i].0 as usize][filter[i].1 as usize] == next_value {
            count += 1;
        };
    }
    return count > 1;
}

fn main() {
    // INIT WITH THESE PARAMETERS
    let HEIGHT: usize = 200;
    let WIDTH: usize = 200;
    let GENS: usize = 100;
    let MAXVAL: u8 = 4;
    let h: usize = HEIGHT - 1;
    let w: usize = WIDTH - 1;
    // INIT ARRAYS AND RNG ENGINE
    let mut rng = rand::thread_rng();
    // INIT pallete
    let mut pallete: [[u8; 3]; 5] = [[0; 3]; 5];
    for i in 0..pallete.len() {
        for j in 0..3 {
            pallete[i][j] = rng.gen_range(0..254) as u8;
        }
    }
    // INIT POPN WITH RANDOM NUMBERS IN RANGE. USING CERTAIN COMBINATIONS OF
    // PROBABILITIES FOR THE DISTRIBUTION FOR RNG WILL PRODUCE MORE CONSISTENT
    // SPIRALS WITH LESS DEVOURING PYRAMIDS.
    let mut popn: Vec<Vec<u8>> = vec![];
    for _ in 0..HEIGHT {
        let mut row: Vec<u8> = vec![];
        for _ in 0..WIDTH {
            row.push(rng.gen_range(0..MAXVAL));
        }
        popn.push(row);
    }
    let mut next_value: u8;
    let mut all_array: Vec<Vec<Vec<u8>>> = vec![];
    let mut new_popn: Vec<Vec<u8>> = popn.clone();
    use std::time::Instant;
    let now = Instant::now();
    for gen in 0..GENS {
        println!("Generation: {}", gen);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                next_value = popn[i][j] + 1;
                if next_value == MAXVAL {
                    next_value = 0;
                }
                if update(&popn, h, w, i as i32, j as i32, next_value) {
                    new_popn[i][j] = next_value;
                }
            }
        }
        popn = new_popn.clone();
        all_array.push(popn.clone());
    }
    let elapsed = now.elapsed();
    println!(
        "Elapsed: {:.2?}. Time Per Iteration {:.2?}",
        elapsed,
        elapsed / GENS as u32
    );
    // WAS TOO LAZY TO FIND A BETTER SOLUTION. ARRAY TO GIF BLOCK USING VEC OF ALL COMPUTED STATES . SUPER SLOW.
    let mut image = File::create("done.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, &[]).unwrap();
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let mut frame: Frame<'static>;
    for i in 0..all_array.len() {
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb(pallete[all_array[i][x as usize][y as usize] as usize]);
        }
        frame = gif::Frame::from_rgb(WIDTH as u16, HEIGHT as u16, &mut *imgbuf);
        // println!("{:?}", popn);
        encoder.write_frame(&frame).unwrap();
    }
}
