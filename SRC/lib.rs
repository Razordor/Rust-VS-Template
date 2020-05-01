//see Rust book for information on how to do unit testing:
//https://doc.rust-lang.org/book/ch11-01-writing-tests.html

#[cfg(test)]
mod tests{
	#[test]
	fn example_test0(){
		let x = 5;
		assert_eq!(x, 5);
	}

	#[test]
	fn example_test1(){
		let x = "hello world";
		assert_eq!(x, "ello world");
	}
}