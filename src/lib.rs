// based on https://github.com/kmanley/base31/blob/master/base31.go by kmanley
// proted from golang to rust by (c) Bernhard Tittelbach 2022
// MIT License

use phf::phf_map;
use std::string::String;

static BASE31 : [char; 31] = [
'2', '3', '4', '5', '6', '7', '8', '9',
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S',
'T', 'U', 'V', 'W', 'X', 'Y', 'Z',];



static B31_INDEX: phf::Map<char, u64> = phf_map! {
			'2'=> 0, '3'=> 1, '4'=> 2, '5'=> 3, '6'=> 4,
			'7'=> 5, '8'=> 6, '9'=> 7, 'A'=> 8, 'B'=> 9,
			'C'=> 10, 'D'=> 11, 'E'=> 12, 'F'=> 13, 'G'=> 14,
			'H'=> 15, 'J'=> 16, 'K'=> 17, 'M'=> 18, 'N'=> 19,
			'P'=> 20, 'Q'=> 21, 'R'=> 22, 'S'=> 23, 'T'=> 24,
			'U'=> 25, 'V'=> 26, 'W'=> 27, 'X'=> 28, 'Y'=> 29,
			'Z'=> 30, 'a'=> 8, 'b'=> 9, 'c'=> 10, 'd'=> 11,
			'e'=> 12, 'f'=> 13, 'g'=> 14, 'h'=> 15, 'j'=> 16,
			'k'=> 17, 'm'=> 18, 'n'=> 19, 'p'=> 20, 'q'=> 21,
			'r'=> 22, 's'=> 23, 't'=> 24, 'u'=> 25, 'v'=> 26,
			'w'=> 27, 'x'=> 28, 'y'=> 29, 'z'=> 30,
};


pub fn encode(mut value: u64) -> String {
	let mut res : [char;16] = ['\0';16];
	let mut i = res.len() -1;
	while value != 0 {
		res[i] = BASE31[(value % 31) as usize];
		value /= 31;
		i -= 1;
	}
	res[i+1..].iter().collect::<String>() //cloned().collect
}


pub fn decode(s : String) -> u64 {
	let mut res : u64 = 0;
	let mut char_rev_iterator = s.chars().rev();
	for idx in 0..s.len() {
		let c : char = char_rev_iterator.next().unwrap();
		let base: u64 = 31;
		let byte_offset = B31_INDEX.get(&c).cloned().unwrap_or(0);
		res += byte_offset * base.pow(idx as u32);
	}
	res
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::iter::zip;

	static RAW : [u64; 13] = [0, 50, 100, 999, 1000, 1111, 5959, 99999, 123456789,
		(i64::MAX as u64) / 2048, (i64::MAX as u64) / 512, i64::MAX as u64, u64::MAX];

	const ENCODED: &'static [&'static str] = &["", "3N", "59", "339", "33A", "36V", "889", "5D3T",
		"6BQ524", "7HCE7EMPVB5", "QZCKQNEQE7H", "DR25MB7KMC659", "SF297MD67PA8H"];

	#[test]
	fn test_encode() {
		for (raw, encoded) in zip(RAW,ENCODED) {
			assert_eq!(encoded.to_string(), encode(raw));
		}
		// Ok(())
	}

	#[test]
	fn test_decode() {
		for (raw, encoded) in zip(RAW,ENCODED) {
			assert_eq!(raw, decode(encoded.to_string()));
			assert_eq!(raw, decode(encoded.to_string().to_lowercase()));
		}
	// Ok(())
	}
}
