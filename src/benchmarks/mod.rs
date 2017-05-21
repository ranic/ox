/*
  Source: benchmarks/mod.rs
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

extern crate test;

#[cfg(test)]
mod tests {
  #[bench]
  fn bench_add_two(b: &mut Bencher) {
      b.iter(|| {
        add_two(2)
      });
  }
}
