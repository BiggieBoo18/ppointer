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


    // by @tatsuya6502さん
    #[test]
    fn test() {
        let y = 5;
        let x = 5;
        // 参照先となるVecがdropされないよう、vを束縛する（Vec<Vec<f64>>型）
        let mut v = vec![vec![0.1; x]; y];
        // initに渡すためのベースになるVecを構築する
	// これもdropされないよう、vpを束縛する（Vec<*mut f64>型）
        let mut vp = v.iter_mut().map(|inner| inner.as_mut_ptr()).collect::<Vec<_>>();
        // vpのVecからinitに渡すポインタを作る（*mut *mut f64型）
        let vp = vp.as_mut_ptr();
        println!("{}", t(&vp));
        unsafe {
            init(vp);
        }
        // 2つ目のvpが束縛されたポインタがここでdropされる
        // 1つ目のvpが束縛されたVec<mut* f64>がここでdropされる
        // vが束縛されたVec<Vec<f64>>がここでdropされる
    }
}
