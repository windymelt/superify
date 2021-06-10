use engiffen::*;
use image::imageops::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        exit_with_message("Please provide image file path");
    }
    let filename = &args[1];
    let outfilename = format!("{}.animated.gif", filename);

    let img = match image::open(filename) {
        Ok(img) => img,
        Err(_) => return exit_with_message("Failed to open file"),
    };

    for i in 0..24 {
        let img2 = colorops::huerotate(&img, 15 * i); // 24frameで360になる
        match img2.save(format!("out_{}.png", i)) {
            Ok(_) => print!("."),
            Err(_) => exit_with_message("Failed to save image"),
        }
    }
    println!();
    let paths = vec![
        "out_0.png",
        "out_1.png",
        "out_2.png",
        "out_3.png",
        "out_4.png",
        "out_5.png",
        "out_6.png",
        "out_7.png",
        "out_8.png",
        "out_9.png",
        "out_10.png",
        "out_11.png",
        "out_12.png",
        "out_13.png",
        "out_14.png",
        "out_15.png",
        "out_16.png",
        "out_17.png",
        "out_18.png",
        "out_19.png",
        "out_20.png",
        "out_21.png",
        "out_22.png",
        "out_23.png",
    ];
    let images = engiffen::load_images(&paths);
    let gif = match engiffen(&images, 24, Quantizer::NeuQuant(2)) {
        Ok(g) => g,
        Err(_) => return exit_with_message("making gif failed"),
    };
    let mut outfile = match File::create(&outfilename) {
        Ok(f) => f,
        Err(_) => return exit_with_message(&format!("failed to create {}", &outfilename)),
    };
    match gif.write(&mut outfile) {
        Ok(_) => (),
        Err(_) => return exit_with_message(&format!("failed to write {}", &outfilename)),
    };
    for p in paths {
        match std::fs::remove_file(p) {
            Ok(_) => (),
            Err(_) => return exit_with_message(&format!("failed to remove temporary file {}", p)),
        };
        print!(".");
    }
    println!("done!");
}

fn exit_with_message(message: &str) {
    eprintln!("*** {}", message);
    std::process::exit(1);
}
