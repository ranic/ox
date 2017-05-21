/*
  Source: parser/mod.rs
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

use std::io::Read;
use std::net::TcpStream;
use consts;
use util;

pub enum Cmd {
  Set {key: String, value: Vec<u8>},
  Get {key: String},
  Del {key: String},
  Unknown {message: String},
}

impl Cmd {
  pub fn new(mut stream: TcpStream) -> Cmd {
    let mut buf = [0 as u8; consts::BUF_SIZE];
    match stream.read(&mut buf[..]) {
      Ok(n) => {
        let cmd = String::from_utf8(buf[0..3].to_vec()).unwrap();
        let i = util::find_space(&buf[4..], n-4).unwrap();
        let key = String::from_utf8(buf[4..i].to_vec()).unwrap();
        let mut value: Vec<u8> = Vec::new();
        let mut i = i + 1;
        loop {
          for idx in i..n {
            value.push(buf[idx]);
          }
          if n < consts::BUF_SIZE {
            break;
          }
          i = 0;
        }
        return match cmd.to_lowercase().as_ref() {
          "set" => Cmd::Set {key, value},
          "get" => Cmd::Get {key},
          "del" => Cmd::Del {key},
          _ => Cmd::Unknown {message: String::from("TODO")},
        };
      }
      Err(e) => {println!("Error in stream.read: {}", e); Cmd::Unknown {message: String::from("TODO")}}
    }

  }
}
