fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
	let mut rev = vec![];
	for x in xs {
		rev.insert(0, x.clone())
	}
	rev
}

pub fn add(x: i32, y: i32) -> i32 {
	return x + y;
}

fn sqrt(number: f64) -> Result<f64, String> {
	if number >= 0.0 {
		Ok(number.powf(0.5))
	} else {
		Err("negative floats don't have square roots".to_owned())
	}
}

fn main() {
	add(1, 2);
	let lst = [1, 2, 3];
	reverse(&lst);
	let res = sqrt(9.0);
	println!("Hello World!");
	println!("{:?}", res);
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	use quickcheck_macros::quickcheck;

	#[test]
	fn test_add() {
		assert_eq!(add(1, 2), 3);
	}

	#[test]
	#[ignore]
	fn test_sqrt() -> Result<(), String> {
		let x = 4.0;
		assert_eq!(sqrt(x)?.powf(2.0), x);
		Ok(())
	}

	#[quickcheck]
	fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
		xs == reverse(&reverse(&xs))
	}
}
