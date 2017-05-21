/*
  Source: main.rs
  Copyright (C) 2017 Akshay Nanavati <https://github.com/akshaynanavati>

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
#![feature(test)]

extern crate ini;
mod hash;
mod parser;
mod util;
mod consts;
mod benchmarks;

use std::net::{TcpListener, TcpStream};
use std::env;
use ini::Ini;

fn handle_client(hash_table: &mut hash::vec::Hash<Vec<u8>>, stream: TcpStream) {
  match parser::Cmd::new(stream) {
    parser::Cmd::Set {key, value} => hash_table.set(key, value),
    parser::Cmd::Get {key} => {
      hash_table.get(key);
      // TODO write back
    },
    parser::Cmd::Del {key} => {
      hash_table.del(key);
      // TODO write back
    },
    parser::Cmd::Unknown {message} => {
      // TODO write back error
      println!("unknown: {}", message);
    },
  };
}

fn main() {
  // Get configuration
  let args: Vec<_> = env::args().collect();
  let conf_file: &str;
  if args.len() < 2 {
    conf_file = consts::DEFAULT_CONFIG_FILE;
  } else {
    conf_file = &args[1];
  }
  let conf = Ini::load_from_file(&conf_file).unwrap();
  let server_conf = conf.section(Some("server")).unwrap();
  let port = server_conf.get("port").unwrap();
  let hash_conf = conf.section(Some("hash")).unwrap();
  let buckets: usize = hash_conf.get("buckets").unwrap().parse().unwrap();

  let mut hash_table: hash::vec::Hash<Vec<u8>> = hash::vec::Hash::new(buckets);
  let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

  // accept connections and process them serially
  for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
            handle_client(&mut hash_table, stream);
        }
        Err(e) => { println!("{}", e); }
    }
  }
}
