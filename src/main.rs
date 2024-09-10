use core::str;
use std::{error::Error, io::{self, Write}, process::Command};

fn simple() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("garmin-converted/tracks.csv")
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

    let output = Command::new("/home/victor/tcx-to-csv/bin/Release/net8.0/linux-x64/tcx-to-csv")
        .arg("-input-folder=/mnt/c/Users/voodo/Downloads/tcx-test")
        // .arg("/mnt/c/Users/voodo/Downloads/tcx-test/activity_16973805783.tcx")
        .output()
        .expect("Failed to run tcx-to-csv converter");

    println!("{:?}", output.stdout);
    if output.status.success() {
        // default output folder should be ./out
        if let Err(err) = simple() {
            println!("error running example: {}", err);
        }
    }
}
