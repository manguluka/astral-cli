extern crate clap;
extern crate astral;
extern crate chrono;
extern crate colored;
extern crate geocode;
// extern crate termion;
// extern crate tui;
#[macro_use] extern crate prettytable;

use astral::{moon,planet,star};
use clap::{App, Arg, SubCommand,AppSettings};
use colored::*;

mod util;
use util::*;

fn main() {
    let app = App::new("astral")
        .setting(AppSettings::AllowLeadingHyphen)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            Arg::with_name("location")
                    .help("Location in the form of 'lat,lon', eg '38.44043,-122.71405'")
                    .takes_value(true)
                    .short("l") 
                    .long("location"),
            Arg::with_name("date")
                    .help("Date in rfc3339 format, eg '1996-12-19T16:39:57-08:00'")
                    .takes_value(true)
                    .short("d") 
                    .long("date"),

        ])
        .subcommand(
            SubCommand::with_name("all")
                .about("Display all available information."),
        )
        .subcommand(
            SubCommand::with_name("moon")
                .about("Display information on Moon."),
        )
        .subcommand(
            SubCommand::with_name("sun")
                .about("Display information on Sun."),
        )
        .subcommand(
            SubCommand::with_name("planet")
                .about("Information on planets of the local solar system")
                .arg(Arg::with_name("name")
                    .required(true)
                    .index(1)
                    .help("Planet name(mercury|venus|mars|jupiter|saturn|neptune|uranus)")
                )
        )
        .subcommand(
            SubCommand::with_name("star")
                .about("Information on stars")
                .arg(Arg::with_name("name")
                    .required(true)
                    .index(1)
                    .help("Star name(Sirius|Polaris)")
                )
        )
        .subcommand(
            SubCommand::with_name("geocode")
                .about("Get lat/long of location.")
                .arg(Arg::with_name("place-name")
                    .required(true)
                    .index(1)
                    .help("Location name, eg 'Santa rosa, Ca'")
                )
                .arg(Arg::with_name("service")
                    .long("service")
                    .default_value("dstoolkit")
                    .help("Geocoding service to use (google|dstoolkit)")
                )
               
        );

    let app_matches = app.clone().get_matches();

    let location = get_location_from_arg(app_matches.value_of("location"));
    let date = get_date_from_arg(app_matches.value_of("date"));
    let julian_day = astral::util::to_julian(date);

    match app_matches.subcommand() {
        ("all", Some(_)) => {
            print_date_location(date,location);
            command_all(julian_day,location);
        }
        ("moon", Some(_)) => {
            print_date_location(date,location);
            let lunar_info = moon::get_lunar_info(julian_day,location);
            print_lunar_info(lunar_info);
        }
        ("sun", Some(_)) => {

        }
        ("planet", Some(planet_matches)) => {
            let planet_info = planet::get_celestial_position(julian_day,planet_matches.value_of("name").unwrap());
            println!("{:?}", planet_info);
        }
        ("star", Some(matches)) => {
            let info = star::get_celestial_position(julian_day,matches.value_of("name").unwrap());
            println!("{:?}", info);
        }
        ("geocode", Some(geocode_matches)) => {
            let place_name = geocode_matches.value_of("place-name").unwrap();
            let results = match geocode_matches.value_of("service").unwrap() {
                "google" => geocode::google::geocode(place_name),
                "dstoolkit" => geocode::ds_toolkit::geocode(place_name),
                _ => panic!("Geocoding service specified not available."),
            };
            match results {
                Ok(result) => println!("Found Location({}). Use '--location {},{}'", result.formatted_address, result.location.lat, result.location.lng,),
                Err(err) => println!("Error: {}",err),
            }
        }
        ("", None) => {
            println!("{}","No command provided.\nUsing default(`astral all`). Run `astral help` for available commands".bold());
            println!();
        },
        _ => unreachable!(), 
    }
}
