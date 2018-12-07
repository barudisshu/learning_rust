#![feature(core_intrinsics)]

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    let _array1 = ['x', 'y', 'z'];
    let _array2 = ['x', 'y'];

    let _vector1: Vec<char> = vec!['x', 'y', 'z'];
    let _vector2: Vec<char> = vec!['x', 'y'];

    print_type_of(&_array1);
    print_type_of(&_array2);
    print_type_of(&_vector1);
    print_type_of(&_vector2);
}
