use std::{io::{ BufWriter, Write}, fs::File, time::Instant, collections::{HashMap}, path::Path};
use cap_placement::{calculate_placement};
use models::Placement;

mod symbols;
mod vias;
mod nets;
mod cap_placement;
mod models;

fn main() {
    println!("------------------------------------------------");
    println!("Start to calculate the capacitor arrangement...");

    let start = Instant::now();
    let mut vias_file: String = String::new();
    if Path::new("./vias.txt").exists() {
        vias_file = "./vias.txt".to_string();
    }
    let power_nets = nets::read_nets("nets.txt");
    let mut net_map = HashMap::new();
    let mut cap_map = symbols::read_symbols(&mut net_map);
    let mut vias = vias::read_vias(&vias_file, power_nets.clone());
    let cap_place_map = calculate_placement(&mut vias, &mut cap_map, &net_map);
    create_placement_file(cap_place_map.clone());
    let elapsed = start.elapsed().as_millis();
    println!("\nCalculate completed in {elapsed} ms, total {:?}, place {:?}.", cap_map.len(), cap_place_map.len());
    pause();
}

fn create_placement_file (cap_place_map: HashMap<String, Placement>) {
    let filename = "place.txt";
    let file = File::create(filename).unwrap();
    let mut buf_writer= BufWriter::new(file);
    buf_writer.write(b"UUNITS = MILS\n").expect("write placement file error!");

    for (_key, value) in cap_place_map {
        let x =  (value.x_axis*100.0).round()/100.0;
        let y =  (value.y_axis*100.0).round()/100.0;
        let row = vec![value.refdes, x.to_string(), y.to_string(), value.rotation, value.mirror, value.symbol_name];
        let mut line = String::new();
        for d in row { line = line + &d + "\t"; }
        line += "\n";
        buf_writer.write(line.as_bytes()).expect("placement file write error!");
    }
}

// pause the cmd window
fn pause() {
    println!("\nPress ENTER to close the window...");
    let buffer = &mut [0u8];
    std::io::Read::read_exact(&mut std::io::stdin(), buffer).unwrap();
}