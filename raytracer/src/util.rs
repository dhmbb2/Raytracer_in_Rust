pub mod const_value;
pub mod ray;
pub mod interval;
pub mod vec3;
pub mod bvh;


// For debugging
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}