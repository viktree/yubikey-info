fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
	let mut rev = vec![];
	for x in xs {
		rev.insert(0, x.clone())
	}
	rev
}

fn add(x: i32, y: i32) -> i32 {
	return x + y;
}

fn main() {
	add(1, 2);
	let lst = [1, 2, 3];
	reverse(&lst);
	println!("Hello World!");
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

	#[quickcheck]
	fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
		xs == reverse(&reverse(&xs))
	}
}
