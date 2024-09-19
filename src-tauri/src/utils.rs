use std::{fs, io::Write, path::Path};

use calamine::{open_workbook, DataType, Reader, Xls};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Point {
    num: u16,
    name: String,
    r#type: String,
    odo: u32,
    lat: f32,
    lon: f32,
    max_speed: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Race {
    code: String,
    race_number: String,
    serial_number: String,
    event_name: String,
    race_name: String,
    sets: Settings,
    types: Vec<PointType>,
    points: Vec<Point>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Settings {
    total: u16,
    max_speed: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PointType {
    caption: String,
    default_rad: u16,
    is_open: bool,
    ghost: bool,
    in_game: bool,
    arrow_threshold: u32,
    max_speed: u16,
}

pub fn convert(file: &str, path: &str) {
    
    let mut wb: Xls<_> = open_workbook(file).expect("не могу открыть исходный файл");
    let sheets_names = wb.sheet_names();

    let mut point = Point {
        num: 0,
        name: "default".to_string(),
        r#type: "DEF".to_string(),
        odo: 0,
        lat: 0.0,
        lon: 0.0,
        max_speed: 0,
    };

    let point_types_captions = [
        "WPV", "WPM", "WPS", "WPE", "DSS", "FZ", "DZ", "WPC", "ASS", "default",
    ];
    let mut point_types: Vec<PointType> = vec![];
    let mut races: Vec<Race> = vec![];

    let sets = Settings {
        total: 0,
        max_speed: 170,
    };
    for t in point_types_captions {
        let mut pt = PointType {
            caption: t.to_string(),
            default_rad: 50,
            is_open: false,
            ghost: false,
            in_game: true,
            arrow_threshold: 800,
            max_speed: sets.max_speed,
        };
        match t {
            "MPV" => {
                pt.is_open = true;
                pt.in_game = false;
            }
            "WPM" => {
                pt.arrow_threshold = 800;
            }
            "WPS" => {
                pt.default_rad = 10;
                pt.arrow_threshold = 1000;
            }
            "WPE" => {
                pt.arrow_threshold = 5000;
            }
            "DSS" => {
                pt.in_game = false;
                pt.is_open = true;
            }
            "ASS" => {
                pt.arrow_threshold = 1000;
            }
            "FZ" => {
                pt.default_rad = 30;
                pt.is_open = true;
                pt.max_speed = 50;
            }
            "WPC" => {
                pt.is_open = true;
                pt.ghost = true;
            }
            _ => {}
        }
        point_types.push(pt);
    }

    for sheet_name in sheets_names {
        let mut race = Race {
            code: "0000".to_string(),
            race_number: "0".to_string(),
            serial_number: "000000000000".to_string(),
            event_name: "BIG EVENT".to_string(),
            race_name: sheet_name.clone(),
            sets: sets.clone(),
            types: point_types.clone(),
            points: vec![],
        };
        let range = wb.worksheet_range(&sheet_name).unwrap();

        for cell in range.used_cells() {
            if cell.0 == 1 {
                match cell.1 {
                    0 => {
                        race.code = cell.2.get_string().unwrap().to_string();
                    }
                    1 => {
                        race.race_number = cell.2.get_float().unwrap().to_string();
                    }
                    4 => {
                        race.serial_number = cell.2.get_float().unwrap().to_string();
                    }
                    5 => {
                        race.event_name = cell.2.get_string().unwrap().to_string();
                    }
                    _ => {}
                }
            }
            if cell.0 > 4 {
                match cell.1 {
                    0 => unsafe {
                        point.num = cell.2.get_float().unwrap().to_int_unchecked::<u16>();
                    },
                    1 => {
                        point.name = cell.2.get_string().unwrap().to_string();
                    }
                    2 => {
                        point.r#type = cell.2.get_string().unwrap().to_string();
                    }
                    3 => unsafe {
                        point.odo =
                            (cell.2.get_float().unwrap() * 1000.0).to_int_unchecked::<u32>();
                    },
                    4 => {
                        let coordinates = cell.2.get_string().unwrap().to_string();
                        let coords: Vec<&str> = coordinates.split(' ').collect();
                        let lat: f32 = coords[1].parse().unwrap();
                        let lon: f32 = coords[3].parse().unwrap();
                        point.lat = lat;
                        point.lon = lon;
                        point.max_speed = sets.max_speed;
                        for t in race.types.clone() {
                            if t.caption == point.r#type {
                                point.max_speed = t.max_speed;
                            }
                        }
                        race.points.push(point.clone());

                        if cell.0 == range.rows().count() - 1 {
                            race.points.push(point.clone());
                            race.sets.total = (range.rows().count() - 3).try_into().unwrap();
                            // create_file(race.clone());
                            races.push(race.clone())
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    create_file(races.clone(), path)
}

fn create_file(races: Vec<Race>, path: &str) {
    let file_name = format!("{}/config_{}.toml", path, &races.first().unwrap().event_name);

    print!("{}:", &file_name);

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct Day {
        races: Vec<Race>,
    }
    let day = Day { races };

    let toml = toml::to_string_pretty(&day).unwrap().replace(' ', "");
    if Path::new(&file_name).exists() {
        fs::remove_file(&file_name).unwrap();
    }
    let mut file = fs::File::create_new(&file_name).unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    println!("OK");
}
