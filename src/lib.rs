use std::vec;

#[derive(Debug)]
pub struct Queen {
    column: i32,
    row: i32,
    n: u32,
    current: i32
}

impl Queen {
	pub fn new(column: i32, row: i32, n: u32) -> Queen {
		Queen{column, row, n, current: 0}
	}
}

impl Iterator for Queen {
	type Item = Queen;

	fn next(&mut self) -> Option<Queen> {
		self.current += 1;
		if self.current > (self.n as i32) {
			return None;
		}
		Some(Queen::new(self.column + 1, self.current, self.n))
	}
}

impl PartialEq for Queen {
	fn eq(&self, other: &Queen) -> bool {
		return self.column == other.column && self.row == other.row;
	}
}

impl Clone for Queen {
	fn clone(&self) -> Queen {
		Queen{column: self.column, row: self.row, n: self.n, current: self.current}
	}
}

fn reject(solution: &vec::Vec<Queen>, candidate: &Queen) -> bool {
	let column = candidate.column;
	let row = candidate.row;
	for queen in solution.iter() {
		let r = queen.row;
		let c = queen.column	;
		if (row == r) || (column == c) || (row + column == r + c) || (row - column == r - c) {
			return true;
		}
	}
	false
}

fn accept(solution: &vec::Vec<Queen>, n: u32) -> bool {
	solution.len() == (n as usize)
}

pub fn backtrack(mut fcg: Queen, n: u32) -> u32 {
	let mut root = fcg;
	let mut stack: vec::Vec<Queen> = vec::Vec::new();
	let mut found = 0;
	loop {
		while let Some(candidate) = root.next() {
			if reject(&stack, &candidate) {
				continue;
			}
			stack.push(root);
			root = candidate;
			if accept(&stack, n) {
				found += 1;
				stack.pop();
				continue;
			}
		}
		if stack.len() == 0 {
			break;
		}
		root = stack.pop().expect("bounds underflow");
		// root = stack.pop().expect("bounds underflow");
		// if let Some(candidate) = root.next() {
		// 	if reject(&solution, &candidate) {
		// 		continue;
		// 	}
		// 	solution.push(candidate.clone());
		// 	if accept(&solution, n) {
		// 		found += 1;
		// 		solution.pop();
		// 		continue;
		// 	}
		// 	stack.push(root.clone());
		// 	root = candidate;
		// } else {
		// 	if stack.len() == 0 {
		// 		break;
		// 	}
			// solution.pop();
			
		// }
	}
	found
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn must_reject() {
// 		let n = 4;
// 		assert!(reject(&vec![&Queen::new(1, 1, n)], &Queen::new(2, 2, n)));
// 		assert!(reject(&vec![&Queen::new(1, 1, n), &Queen::new(2, 4, n)], &Queen::new(3, 4, n)));
// 		assert!(reject(&vec![&Queen::new(1, 1, n), &Queen::new(2, 4, n), &Queen::new(3, 4, n)], &Queen::new(4, 3, n)));
// 	}

// // 1-
// //  0  0  1  0 
// //  1  0  0  0 
// //  0  0  0  1 
// //  0  1  0  0 
// // 2-
// //  0  1  0  0 
// //  0  0  0  1 
// //  1  0  0  0 
// //  0  0  1  0 
// 	#[test]
// 	fn must_not_reject() {
// 		let n = 4;
// 		assert!(!reject(&vec![&Queen::new(1, 2, n)], &Queen::new(2, 4, n)));
// 		assert!(!reject(&vec![&Queen::new(1, 2, n), &Queen::new(2, 4, n)], &Queen::new(3, 1, n)));
// 		assert!(!reject(&vec![&Queen::new(1, 2, n), &Queen::new(2, 4, n), &Queen::new(3, 1, n)], &Queen::new(4, 3, n)));

// 		assert!(!reject(&vec![&Queen::new(1, 3, n)], &Queen::new(2, 1, n)));
// 		assert!(!reject(&vec![&Queen::new(1, 3, n), &Queen::new(2, 1, n)], &Queen::new(3, 4, n)));
// 		assert!(!reject(&vec![&Queen::new(1, 3, n), &Queen::new(2, 1, n), &Queen::new(3, 4, n)], &Queen::new(4, 2, n)));
// 	}

// 	#[test]
// 	fn must_accept() {
// 		let n = 4;
// 	    assert!(accept(&vec![&Queen::new(1, 3, n), &Queen::new(2, 1, n), &Queen::new(3, 4, n), &Queen::new(4, 2, n)], 4));
// 	}

// 	#[test]
// 	fn must_not_accept() {
// 		let n = 4;
// 		assert!(!accept(&vec![&Queen::new(1, 3, n), &Queen::new(2, 1, n), &Queen::new(3, 4, n)], 4));
// 	}

// 	#[test]
// 	fn correct_children() {
// 		let n = 4;
// 	    let fcg = Queen::new(0, 0, n);
// 	    let n = 4;
// 	    let expected = vec![Queen::new(1, 1, n), Queen::new(1, 2, n), Queen::new(1, 3, n), Queen::new(1, 4, n)];
// 	    let mut got: vec::Vec<Queen> = vec::Vec::new();
// 	    for queen in fcg {
// 	    	got.push(queen);
// 	    }
// 	    assert_eq!(expected, got);
// 	}
// }