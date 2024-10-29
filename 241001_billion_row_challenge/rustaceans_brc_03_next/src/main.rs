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
// vec 10_000_000 : 8.63 s
// vec 10_000_000 : 8.63 s

fn read_line(reader: &mut impl BufRead, mut buf_meter: &mut Vec<u8>, mut buf_city: &mut Vec<u8>) -> Option<f32> {
    // let mut buf_city = Vec::with_capacity(50);
    //let mut buf_meter = Vec::with_capacity(50);

    let read_bytes = reader.read_until(';' as u8, &mut buf_city).unwrap();
    if read_bytes == 0 {
        None
    } else {
        reader.read_until('\n' as u8, &mut buf_meter).unwrap();

        Some(0.3) //f32::parinse(String::from(buf_meter.as_slice()))))
    }
}

fn main() {
    let now = Instant::now();

    let mut stations= HashMap::new();
    let file = File::open("../measurements_10mio.txt").unwrap();
    let mut reader = BufReader::with_capacity(16000, file);

    let mut buf_city = Vec::with_capacity(50);
    let mut buf_meter = Vec::with_capacity(50);

    while let Some(meter) = read_line(&mut reader, &mut buf_meter, &mut buf_city) {
        // println!("readline: {:?}", buf_city);
        update_stations(&mut stations, (&buf_city, meter));
        buf_meter.truncate(0);
        buf_city.truncate(0);
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
            "Name: {:?}, {}/{} Avg {}",
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

fn update_stations(stations: &mut HashMap<Vec<u8>, Station>, read_value: (&Vec<u8>, f32)) {
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
                read_value.0.to_owned(),
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
