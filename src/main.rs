use clap::{Parser, ArgGroup};
use std::{error::Error, process::Command};

const HEART_RATE_CSV_INDEX: usize = 7;
const TCX_TO_CSV_EXECUTABLE_PATH: &str = "/home/victor/tcx-to-csv/bin/Release/net8.0/linux-x64/tcx-to-csv";

#[derive(Parser, Debug)]
#[command(version, about = "A command line app to calculate first and second half HR from tcx files", long_about = None)]
#[command(group(ArgGroup::new("group").required(true).args(&["input_folder", "full_path"])))]
struct Cli {
    /// Directory that contains the tcx file; NOTE: assumes a leading path of: /mnt/c/Users/voodo/Downloads
    // TODO: make this input accept just a tcx file, not a dir, or optionally a dir of tcx files
    // TODO: don't assume windows Downloads, make this a cleaner solution
    #[arg(short, long)]
    input_folder: Option<String>,
    /// Absolute path to directory that contains tcx file
    #[arg(short, long)]
    full_path: Option<String>,
}

fn calculate_average(values: &[u32]) -> f32 {
    let mut sum = 0;
    for i in values {
        sum += i;
    }

    return sum as f32 / values.len() as f32;
}

fn calculate_percent_increase(a: f32, b: f32) -> f32 {
    ((b / a) - 1.0) * 100.0
}

fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("out/tracks.csv")
        .unwrap();

    // let mut count = 0;
    let mut heart_rates = vec![];
    for result in reader.records() {
        let record = result?;
        let heart_rate = record.get(HEART_RATE_CSV_INDEX).unwrap().parse::<u32>().unwrap();

        heart_rates.push(heart_rate);
        // count += 1;
    }

    // println!("{:?}", count);
    let half_index = heart_rates.len() / 2;
    let (first_half, second_half) = heart_rates.split_at(half_index);

    let first_half_avg = calculate_average(first_half);
    let second_half_avg = calculate_average(second_half);

    println!("First half average HR: {}\nSecond half average HR: {}", first_half_avg, second_half_avg);
    println!("Percent increase: {}", calculate_percent_increase(first_half_avg, second_half_avg));

    Ok(())
}

fn run_tcx_to_csv_executable(input_arg: &str) {
    let output = Command::new(TCX_TO_CSV_EXECUTABLE_PATH)
        .arg(input_arg)
        // .arg("/mnt/c/Users/voodo/Downloads/tcx-test/activity_16973805783.tcx")
        .output()
        .expect("Failed to run tcx-to-csv converter");

    if output.status.success() {
        // default output folder should be ./out
        if let Err(err) = parse_csv() {
            println!("error running program: {}", err);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    if let Some(input_dir) = cli.input_folder {
        run_tcx_to_csv_executable(&format!("-input-folder=/mnt/c/Users/voodo/Downloads/{}", input_dir));
    } else if let Some(input_dir_full_path) = cli.full_path {
        run_tcx_to_csv_executable(&format!("-input-folder={}", input_dir_full_path));
    }
}
