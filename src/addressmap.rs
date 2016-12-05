use std::fs::{self, File};
use std::io::{self, BufReader};
use std::io::prelude::*;

use libc::pid_t;

#[derive(Clone, Debug)]
pub struct AddressMap {
    pub lower_addr: u64,
    pub upper_addr: u64,
    pub path: String,
}


#[derive(Clone, Debug)]
pub struct Maps {
    map: Vec<AddressMap>,
}

impl Maps {
    pub fn resolve(&self, address: u64) -> Option<&AddressMap> {
        self.map.iter().find(|m| m.lower_addr < address && address < m.upper_addr)
    }

    pub fn parse(&self) {}

    pub fn load(pid: pid_t) -> Result<Maps, ()> {
        let file = match File::open(format!("/proc/{:}/maps", pid)) {
            Err(why) => panic!("couldn't open proc map: {}", why),
            Ok(file) => file,
        };

        let mut addressmap = vec![];

        let f = BufReader::new(file);
        for line in f.lines() {
            let l = line.unwrap();

            let (lower, upper, lib) = scan_fmt!(l.as_str(),
                                                "{x}-{x} {*} {*} {*} {*} {}",
                                                String,
                                                String,
                                                String);

            let lower_addr = u64::from_str_radix(&lower.unwrap().as_str(), 16).map_err(|_| ());
            let upper_addr = u64::from_str_radix(&upper.unwrap().as_str(), 16).map_err(|_| ());

            addressmap.push(AddressMap {
                lower_addr: lower_addr.unwrap(),
                upper_addr: upper_addr.unwrap(),
                path: lib.unwrap_or(String::from("")),
            });
        }

        Ok(Maps { map: addressmap })
    }
}
