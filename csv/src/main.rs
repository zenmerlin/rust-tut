fn main() {
    // example program processing csv data
    let data = "\
    name,age
    Charles,43
    Jackson,22
    Jimmy,30
    The dude, 46
    ";

    let records = data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(age) = fields[1].parse::<f32>() {
            println!("{}, {}", name, age);
        }
    }
}
