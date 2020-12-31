use std::fs::OpenOptions;

use clap::{App, ArgMatches, load_yaml};
use colour::e_red_ln;
use image::{ColorType, GenericImageView};
use stl_io::{write_stl};
use heightmap_to_stl::heightmap_to_stl;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let image_file = matches.value_of("input").unwrap();
    let stl_file_name = matches.value_of("output").unwrap();

    let base_height = try_parse_double(&matches, "base_height");
    let model_height = try_parse_double(&matches, "model_height");

    println!("Input image file: {0}", image_file);
    println!("Output stl file: {0}", stl_file_name);
    println!("Base height: {0}", base_height);
    println!("Model height: {0}", model_height);

    println!();
    println!("Start reading image");
    let img = match image::open(image_file) {
        Ok(i) => i,
        Err(_) => {
            e_red_ln!("{0} is not a valid image file!", image_file);
            std::process::exit(1);
        },
    };

    let (width, height) = img.dimensions();

    println!("Start generating heightmap");
    let mut heightmap = vec![vec! [0f32; height as usize]; width as usize];

    let color_type = img.color();
    match color_type {
        ColorType::L8 => {
            for (x, y, luma) in img.to_luma8().enumerate_pixels() {
                let l = (*luma)[0] as f32 / ((2i32).pow(8) as f32 - 1.0);
        
                let local_height = l * model_height;
                heightmap[x as usize][y as usize] = local_height;
            }
        }
        ColorType::L16 => {
            for (x, y, luma) in img.to_luma16().enumerate_pixels() {
                let l = (*luma)[0] as f32 / ((2i32).pow(16) as f32 - 1.0);
        
                let local_height = l * model_height;
                heightmap[x as usize][y as usize] = local_height;
            }
        }
        _ => {
            e_red_ln!("image should be a Luma image!");
            std::process::exit(1);
        }
    }    

    println!("Start generating mesh");
    let mesh = heightmap_to_stl(heightmap, base_height);

    println!("Start writing to file");
    let mut stl_file = OpenOptions::new().write(true).create(true).open(stl_file_name).unwrap();
    write_stl(&mut stl_file, mesh.iter()).unwrap();
}

fn try_parse_double (matches: &ArgMatches, name: &str) -> f32 {
    let string_value = matches.value_of(name).unwrap();
    match string_value.parse::<f32>() {
        Ok(n) => n,
        Err(_) => {
            e_red_ln!("{0} should be a double!", name);
            std::process::exit(1);
        },
    }
}