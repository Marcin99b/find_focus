use std::{fs, io::Error};

use chrono::{DateTime, Local};

struct FocusedTime {
    from: DateTime<Local>,
    to: DateTime<Local>,
    minutes: i64,
    activity: String,
}

fn main() {
    let records = parse_history().unwrap();
    for record in records {
        println!("{}", record.from)
    }
}

fn parse_history() -> Result<Vec<FocusedTime>, Error> {
    let date_format = "%a %b %d %T GMT%z %Y";
    if let Ok(content) = fs::read_to_string("c:/focus-logs.csv") {
        let lines = content.split('\n').skip(1);
        let mut vec: Vec<FocusedTime> = Vec::new();
        for line in lines {
            let cleared = line.replace('\"', "");
            if cleared.len() < 10 {
                continue;
            }
            let items = cleared.split(',').collect::<Vec<&str>>();
            println!("{}", &items[0]);
            let from = match DateTime::parse_from_str(items[0], date_format) {
                Ok(x) => x,
                Err(err) => panic!("{:?}", err),
            }.with_timezone(&Local);

            let to = match DateTime::parse_from_str(items[1], date_format) {
                Ok(x) => x,
                Err(err) => panic!("{:?}", err),
            }.with_timezone(&Local);

            let minutes = (to - from).num_minutes();
            let activity = items[2].to_string();

            let record = FocusedTime {
                from,
                to,
                minutes,
                activity
            };

            vec.push(record);
        }
        return Ok(vec);
    }
    else {
        panic!("Cannot parse file");
    }
}