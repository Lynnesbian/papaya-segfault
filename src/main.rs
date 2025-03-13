use papaya::HashSet;

fn main() {
	let map = HashSet::new();
	let mut map = map.pin();

	for i in 0..=50 {
		map.insert(format!("Hello {i}!"));
	}

	map.retain(|s| dbg!(s) == "Hello 33!");
}
