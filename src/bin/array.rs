fn main() {
	let _array: [i32; 3] = [1, 3, 8];
	// Cette boucle va afficher 1, 3, 8
	for x in &_array {
		print!("{} ", x);
	}
}
