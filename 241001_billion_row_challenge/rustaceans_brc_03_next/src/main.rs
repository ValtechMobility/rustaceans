use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Map,
    os::macos::raw::stat,
    time::Instant,
};

#[derive(Debug)]
struct Station {
    // Longest name is 49 Characters in case we want to use bytearrays
    min: f32,
    max: f32,
    sum: f32,
    count: u32,
    avg: f32,
}

// vec 10_000 : ?
// vec 100_000_000 : ?

fn main() {
    let now = Instant::now();

    let mut stations: HashMap<String, Station> = HashMap::new();
    let file = File::open("../measurements_100mio.txt").unwrap();
    let reader = BufReader::with_capacity(2_000_000, file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: (&str, &str) = line.split_once(";").unwrap();
        let name = parts.0;
        let temp = parts.1.parse::<f32>().unwrap();

        update_stations(&mut stations, (name, temp));
    }


    // std::fs::read_to_string("../measurements_100mio.txt")
    //     .unwrap() // panic on possible file-reading errors
    //     .lines() // split the string into an iterator of string slices
    //     .map(parse_line) // make each slice into a string
    //     .for_each(|measurement| update_stations(&mut stations, measurement));

    // stations.iter()
    for (_, station) in stations.iter_mut() {
        station.avg = station.sum / station.count as f32;
        // println!("{};{:.2};{:.2};{:.2}", name, station.min, avg, station.max);
    }


    let mut sts: Vec<_> = Vec::with_capacity(stations.len());
    sts = stations.iter().collect();
    sts.sort_by(|a, b| a.0.cmp(b.0));

    for (name, station) in sts {
        println!(
            "Name: {}, {}/{} Avg {}",
            name, station.min, station.avg, station.max
        );
    }

    let elapsed = now.elapsed();


    println!("Parsing took: {:.2?}", elapsed);
}

fn parse_line(line: &str) -> (&str, f32) {
    let (name, value) = line.split_once(";").unwrap();
    (name, value.parse::<f32>().unwrap())
}

fn update_stations(stations: &mut HashMap<String, Station>, read_value: (&str, f32)) {
    let station = stations.get_mut(read_value.0);
    match station {
        Some(s) => {
            s.count += 1;
            s.sum += read_value.1;
            s.min = s.min.min(read_value.1);
            s.max = s.max.max(read_value.1);
        }
        None => {
            stations.insert(
                read_value.0.to_string(),
                Station {
                    min: read_value.1,
                    max: read_value.1,
                    sum: read_value.1,
                    count: 1,
                    avg: 0.,
                },
            );
        }
    }
}