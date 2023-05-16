use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;

pub fn read_freq_csv(filepath: &str) -> Result<HashMap<String, f64>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<String, f64> = HashMap::new();
    for res_line in reader.lines().skip(1) {
        let row = res_line?;
        let columns: Vec<&str> = row.split(';').collect();

        let word = columns[0];
        // let val1: f64 = columns[1].parse().expect("parse failed");
        // let val2: f64 = columns[2].parse().expect("parse failed");
        // let val3: f64 = columns[3].parse().expect("parse failed");
        let val4: f64 = columns[4].parse().map_err(|_| {
            std::io::Error::new(ErrorKind::Other, "Parsing column 4 of file failed")
        })?;

        // println!("Mot: {}", word);
        let entry = map.get(word);
        if let Some(previous_val) = entry {
            map.insert(word.to_string(), previous_val + val4);
        } else {
            map.insert(word.to_string(), val4);
        }
        // println!("Valeur 4: {}", val4);
    }
    return Ok(map);
}
