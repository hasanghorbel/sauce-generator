extern crate colored;
extern crate rand;
extern crate structopt;

use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use colored::*;
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    let mut message = options.message.split(" ").collect::<Vec<&str>>();
    let scale = 8;
    for _ in 0..scale - (message.len() % scale) {
        message.push("");
    }

    let codes = spread_sauce(options.sauce);
    for i in &codes {
        message.push(i);
        for _ in 1..scale {
            message.push("");
        }
    }

    let file = File::open("h.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let slices = (message.len() / scale) - 1;
    let mut slice_index = 0;
    let mut line_index = 0;
    for line in buf_reader.lines() {
        if line_index >= 15 {
            if slice_index <= slices {
                let spaces = " ".repeat(205 - line.as_ref().unwrap().len());
                print!("{}{}", line.unwrap(), spaces);
                for i in slice_index * scale..(slice_index + 1) * scale {
                    print!("{} ", message[i].bright_magenta().underline());
                }
                print!("\n");
                slice_index += 1;
            } else {
                println!("{}", line.unwrap());
            }
        } else {
            println!("{}", line.unwrap());
        }
        line_index += 1;
    }
}

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "m", long = "message", default_value = "YAMETE KUDASAI!")]
    // Write some message
    message: String,
    #[structopt(short = "s", long = "sauce", default_value = "0")]
    // Generate some sauce
    sauce: u32,
}
fn spread_sauce(sauce: u32) -> Vec<String> {
    let mut sauces = vec![];
    for _ in 0..sauce {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(100000..=450000);
        let url = format!("https://nhentai.to/g/{}", random_num);
        sauces.push(url);
    }
    sauces
}
