use std::{cmp::min, fs::File, io::{BufRead, BufReader}, iter::Map};

#[derive(Debug)]
struct Station {
    name: String,
    min: f32,
    max: f32,
    sum: f32,
    count: u32
}

// vec 10_000 : 4.55 ms
// vec 100_000_000 : 396.70s

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut stations = vec![];

    std::fs::read_to_string("../measurements.txt").unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(parse_line)  // make each slice into a string
    .for_each(|measurement| update_stations(&mut stations, measurement));

    stations.sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
    for station in stations {
        let avg = station.sum / station.count as f32;
        println!("{};{:.2};{:.2};{:.2}", station.name, station.min, avg, station.max);
    }
    
    let elapsed = now.elapsed();

    println!("Parsing took: {:.2?}", elapsed);

}

fn parse_line(line: &str) -> (&str, f32) {
    let (name, value) = line.split_once(";").unwrap();
    (name, value.parse::<f32>().unwrap())
}

fn update_stations(stations: &mut Vec<Station>, read_value: (&str, f32)) {
    let station = stations.iter_mut().find(|s| s.name == read_value.0);
    match station {
        Some(s) => {
            s.count += 1;
            s.sum += read_value.1;
            s.min = s.min.min(read_value.1);
            s.max = s.max.max(read_value.1);
        },
        None => {
            stations.push(Station {
                name: read_value.0.to_string(),
                min: read_value.1,
                max: read_value.1,
                sum: read_value.1,
                count: 1,
            });
        }
    }
}