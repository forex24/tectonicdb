extern crate clap;
extern crate byteorder;
extern crate dtf;

use clap::{Arg, App};
use std::net::TcpStream;
use std::str;
use byteorder::{BigEndian, /*WriteBytesExt, */ ReadBytesExt};
use std::io::{self, Read, Write};

fn main() {
        let matches = App::new("tectonic-cli")
                          .version("0.0.1")
                          .author("Ricky Han <tectonic@rickyhan.com>")
                          .about("command line client for tectonic financial datastore")
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("input")
                               .value_name("INPUT")
                               .help("file to read")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("metadata")
                               .short("m")
                               .long("metadata")
                               .help("read only the metadata"))
                          .arg(Arg::with_name("csv")
                               .short("c")
                               .long("csv")
                               .help("output csv"))
                          .get_matches();
    let input = matches.value_of("input").unwrap();
    let metadata = matches.is_present("metadata");
    let csv = matches.is_present("csv");

    if metadata {
        println!("{}", dtf::read_meta(input));
        return;
    } else {
        if !csv {
            println!("[{}]", dtf::update_vec_to_json(&dtf::decode(input)));
            return;
        }
        println!("{}", dtf::update_vec_to_csv(&dtf::decode(input)));
        return;
    }
}