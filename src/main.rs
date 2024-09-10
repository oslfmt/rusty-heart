use clap::Parser;
use std::{error::Error, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input tcx data file; NOTE: assumes this is in windows Downloads and in a dir
    // TODO: make this input accept just a tcx file, not a dir, or optionally a dir of tcx files
    // TODO: don't assume windows Downloads, make this a cleaner solution
    #[arg(short, long)]
    input: String,
}

fn simple() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("out/tracks.csv")
        .unwrap();

    let mut count = 0;
    let mut heart_rates = vec![];
    for result in reader.records() {
        let record = result?;
        let heart_rate = record.get(7).unwrap().parse::<u32>().unwrap();

        heart_rates.push(heart_rate);
        // println!("{:?}", heart_rate);
        count += 1;
    }

    // println!("{:?}", count);
    let half_index = heart_rates.len() / 2;
    let (first_half, second_half) = heart_rates.split_at(half_index);

    let mut sum = 0;
    for i in first_half {
        sum += i;
    }
    let first_half_avg: f32 = sum as f32 / first_half.len() as f32;

    // println!("{:?}", first_half);
    let mut second_sum = 0;
    for j in second_half {
        second_sum += j;
    }
    let second_half_avg: f32 = second_sum as f32 / second_half.len() as f32;

    println!("First: {}, Second {}", first_half_avg, second_half_avg);

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let input_file = cli.input;
    
    let output = Command::new("/home/victor/tcx-to-csv/bin/Release/net8.0/linux-x64/tcx-to-csv")
        .arg(format!("-input-folder=/mnt/c/Users/voodo/Downloads/{}", input_file))
        // .arg("/mnt/c/Users/voodo/Downloads/tcx-test/activity_16973805783.tcx")
        .output()
        .expect("Failed to run tcx-to-csv converter");

    if output.status.success() {
        // default output folder should be ./out
        if let Err(err) = simple() {
            println!("error running example: {}", err);
        }
    }
}
