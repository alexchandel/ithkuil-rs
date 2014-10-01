#![feature(globs)]

extern crate ithkuil;

use ithkuil::category::*;

fn main() {
	println!("{}", encode_01(decode_01("t").unwrap()))
	println!("{}", encode_02(decode_02("a").unwrap()))
	println!("{}", encode_03(decode_03("h").unwrap()))
	println!("{}", encode_04(decode_04("a").unwrap()))
	println!("{}", encode_08(decode_08("a").unwrap()))
	println!("{}", encode_09(decode_09("wa").unwrap()))
	println!("{}", encode_10(decode_10("l").unwrap()))
	println!("{}", encode_d(decode_d("a").unwrap()))
	println!("{}", encode_e(decode_e("n-n").unwrap()))
	println!("{}", encode_f(decode_f("a").unwrap()))

	println!("{}", decode_01("t").unwrap())
	println!("{}", decode_02("a").unwrap())
	println!("{}", decode_03("h").unwrap())
	println!("{}", decode_04("a").unwrap())
	println!("{}", decode_08("a").unwrap())
	println!("{}", decode_09("wa").unwrap())
	println!("{}", decode_10("l").unwrap())
	println!("{}", decode_d("a").unwrap())
	println!("{}", decode_e("n-n").unwrap())
	println!("{}", decode_f("a").unwrap())
}
