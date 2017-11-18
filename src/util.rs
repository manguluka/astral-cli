use colored::*;
use astral::*;
use astral::coords::*;
use std;
use chrono::prelude::*;
use prettytable::Table;

const DEFAULT_LOCATION: Location = Location {
    lat:38.44043,
    lon:-122.71405,
};

pub fn table(header:(&str,&str,&str),rows: Vec<(&str,f64,f64)>){
    let mut table = Table::new();
    table.add_row(row![header.0,header.1,header.2]);
    for &x in &rows {
        table.add_row(row![x.0,x.1,x.2]);
    }
    table.printstd();
}

pub fn celestial_position_table(rows: Vec<(&str,f64,f64)>){
    let mut table = Table::new();
    let header = ("Object", "Azimuth", "Altitude");
    table.add_row(row![header.0,header.1,header.2]);
    for &x in &rows {
        table.add_row(row![x.0,x.1,x.2]);
    }
    table.printstd();
}

pub fn command_star(julian_day: f64,location: Location,name: &str) {
    let star_data = star::get_data(name.to_string()).unwrap();
    let star_info =  star::get_celestial_position(julian_day, name).unwrap().get_hz_coords(location);
    let  rows: Vec<(&str,f64,f64)> = vec![
        (star_data.proper.as_str(),star_info.az,star_info.alt),
    ];
    celestial_position_table(rows);
}

pub fn command_planet(julian_day: f64,location: Location,name: &str) {
    let info =  planet::get_celestial_position(julian_day, name).unwrap().get_hz_coords(location);
    let  rows: Vec<(&str,f64,f64)> = vec![
        (name,info.az,info.alt),
    ];
    celestial_position_table(rows);
}

pub fn command_all(julian_day: f64,location: Location) {
    let solar_info = sun::get_celestial_position(julian_day).get_hz_coords(location);
    let lunar_info = moon::get_lunar_info(julian_day,location);
    let venus_info =  planet::get_celestial_position(julian_day, "Venus").unwrap().get_hz_coords(location);
    let mercury_info =  planet::get_celestial_position(julian_day, "Mercury").unwrap().get_hz_coords(location);
    let mars_info =  planet::get_celestial_position(julian_day, "Mars").unwrap().get_hz_coords(location);
    let jupiter_info =  planet::get_celestial_position(julian_day, "Jupiter").unwrap().get_hz_coords(location);
    let saturn_info =  planet::get_celestial_position(julian_day, "Saturn").unwrap().get_hz_coords(location);
    let polaris_info =  star::get_celestial_position(julian_day, "Polaris").unwrap().get_hz_coords(location);
    let header = ("Object", "Azimuth", "Altitude");
    let  rows: Vec<(&str,f64,f64)> = vec![
        ("Sun",solar_info.az,solar_info.alt),
        ("Moon",lunar_info.azimuth,lunar_info.altitude),
        ("Venus",venus_info.az,venus_info.alt),
        ("Mercury",mercury_info.az,mercury_info.alt),
        ("Mars",mars_info.az,mars_info.alt),
        ("Jupiter",jupiter_info.az,jupiter_info.alt),
        ("Saturn",saturn_info.az,saturn_info.alt),
        ("Polaris",polaris_info.az,polaris_info.alt),
    ];
    table(header,rows);
}

pub fn print_date_location(date: DateTime<FixedOffset>, location: Location) {
    let is_default_date = (Local::now().timestamp() - date.timestamp()).abs() < 1;
    let default_date_text = if is_default_date {"(default to current time)"} else {""};
    let default_location_text = if location == DEFAULT_LOCATION {"(using default location)"} else {""};
    println!("{}","Displaying information for:".bold());
    println!(
        "Location: {},{} {}", 
        location.lat.to_string().bold(), 
        location.lon.to_string().bold(), 
        default_location_text
    );
    println!("Date: {} {}", date.to_string().bold(), default_date_text);
    println!();
}
pub fn print_lunar_info(info: LunarInfo) {
    println!("{}", "Moon:".bold());
    println!("Percent Illuminated: {:.6}%", info.percent_illuminated.to_string().bold());
    println!("Phase: {}", info.phase_name.to_string().bold());
    println!("Altitude: {:.6}", info.altitude.to_string().bold());
    println!("Azimuth: {:.6}", info.azimuth.to_string().bold());
    print!("{}",info.phase_image);
}

pub fn get_location_from_arg(location_arg: std::option::Option<&str>) -> Location {
    location_arg
        .and_then(|pos|{
            let pos_vec: Vec<f64> = pos.split(',').map(|s| s.parse().unwrap()).collect();
            Some(Location{
                lat:pos_vec[0],
                lon:pos_vec[1],
            })
        })
        .unwrap_or(DEFAULT_LOCATION)
}

pub fn get_date_from_arg(date_arg: std::option::Option<&str>) -> DateTime<FixedOffset> {
    let current_local_datetime = Local::now();
    let default_date = current_local_datetime.with_timezone(current_local_datetime.offset());
    match date_arg {
        Some(pos) => {
            DateTime::parse_from_rfc3339(pos).unwrap()
        },
        None => {
            default_date
        }
    }
}