use std::os::raw::{c_double};

#[link(name = "ppointer")]
extern {
    pub fn init(v: *mut *mut c_double);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::os::raw::{c_double, c_void};
    use crate::init;

    #[allow(dead_code)]
    fn t<T>(_: T) -> &'static str {
    	std::any::type_name::<T>()
    }

    #[test]
    fn test() {
	let y = 5;
	let x = 5;
	let mut v = [[0.1; x]; y];
	let v     = &mut v as *mut _ as *mut *mut c_double;
	println!("{}", t(&v));
	unsafe {
	    init(v);
	}
    }
}
