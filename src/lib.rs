mod utils;

use wasm_bindgen::prelude::*;
//extern crate meval;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ExportedTupleStruct(pub f32, pub f32);

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn f(x: f32, y: f32) -> f32 {
    // println!("In f: x: {} y: {}", &x,&y);
    (x.powf(2.0) + y.powf(2.0)).sin()
}

//pub fn grad_approx(x: f32, y: f32, func: fn(x: f32, y: f32) -> f32) -> (f32, f32) {
    //(1.0,1.0)
//}

pub fn grad(x: f32, y: f32) -> (f32,f32) {
    (2.0*x*(x.powf(2.0) + y.powf(2.0)).cos(),2.0*y*(x.powf(2.0) + y.powf(2.0)).cos())
}

#[allow(unused_assignments)]
pub fn g(x: f32, y: f32, grad: fn(x: f32, y: f32) -> (f32,f32)) -> Box<dyn Fn(f32)->f32> {
    // println!("In g: x: {} y: {}", &x,&y);
    // Input: Startpunkt
    let p1 = (x,y);
    // Berechne Grad an Startpunkt
    let grad_at_start = grad(x,y);
    let p2 = (p1.0 + grad_at_start.0,p1.1 + grad_at_start.1);
    // Bestimme Geradengleichung in R
    let mut m = 0.0;
    let mut n = 0.0;
    if p1.0 != p2.0 {
        m = (p2.1 - p1.1)/(p2.0 - p1.0);
        n = p1.1 - m*p1.0;
        // println!("m: {} n: {}", &m,&n);
        return Box::new(move |x:f32| {
            m*x + n
        });
    } else if p1.1 != p2.1 {
        m = (p2.0 - p1.0)/(p2.1 - p1.1);
        n = p1.0 - m*p1.1;
        return Box::new(move |y:f32| m*y + n);
    } else {
        panic!("x and y values of function are equal! Probably a bad starting point!");
    }
}

pub fn norm(p1: &(f32,f32), p2: &(f32,f32)) -> f32 {
    ((p1.0-p2.0).powf(2.0) + (p1.1-p2.1).powf(2.0)).sqrt()
}

pub fn fib(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 2 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

pub fn fibonacci_search<F: Fn(f32)->f32>(n: i32, a: f32, b: f32, func: F) -> (f32,f32) {
    fn find_sym_k(left: i32, right: i32, k: i32) -> i32 {
        if k > (right-left)/2 {
            return left + (right - k);
        } else if k < (right-left)/2 {
            return right - (k-left);
        } else {
            return k;
        }
    }
    fn fib_search_rec<F: Fn(f32)->f32>(a: f32, b: f32, left: i32, right: i32, k: i32, delta_x: f32, func: F) -> (f32,f32){
        let new_k = find_sym_k(left,right,k);
        if k == new_k {
            return (a+k as f32 * delta_x, func(a+k as f32 * delta_x));
        }
        let x_current = a + k as f32 * delta_x;
        let y_current = func(x_current);
        let x_compare = a + new_k as f32 * delta_x;
        let y_compare = func(x_compare);
        //println!("a:{} b:{} left:{} right:{} k:{}",&a,&b,&left,&right,&k);
        // println!("k: {} new_k: {}", &k, &new_k);
        // println!("x_current:{} y_current:{} x_compare:{} y_compare:{}",&x_current,&y_current,&x_compare,&y_compare);

        if y_current < y_compare {
            if k < new_k {
                return fib_search_rec(a,b,left,new_k,k,delta_x,func);
            } else {
                return fib_search_rec(a,b,new_k,right,k,delta_x,func);
            }
        } else {
            if k < new_k {
                return fib_search_rec(a,b,k,right,new_k,delta_x,func);
            } else {
                return fib_search_rec(a,b,left,k,new_k,delta_x,func);
            }
        }
    }
    let _n = fib(n as usize + 2) as i32;
    let phi = fib(n as usize) as i32;
    let delta_x = (b-a) / (_n as f32);
    return fib_search_rec(a,b,0,_n,phi,delta_x,func);
}

pub fn gradientenanstiegsverfahren(x: f32, y: f32, delta: f32, func: fn(x:f32,y:f32) -> f32, grad: fn(x:f32,y:f32)->(f32,f32)) -> (f32,f32) {
    let mut start = (x,y);
    let mut left = start.0 - delta;
    let mut right = start.0 + delta;
    
    let mut i = 0;
    loop {
        let _g = g(start.0,start.1,grad);
        let fib_func = |v:f32| func(v,_g(v));
        let new_x = fibonacci_search(20,left,right,fib_func).0;
        let min = (new_x,_g(new_x));
        console_log!("{:?}", &min);
        start = min;
        if norm(&start,&min) < 0.001 { return min }
        i += 1;
        left = start.0 - delta;
        right = start.0 + delta;

        if i>100 { return min }
    }

}

#[wasm_bindgen]
pub fn get_min(x: f32, y: f32, delta: f32) -> ExportedTupleStruct {
    let tup = gradientenanstiegsverfahren(x,y,delta,f,grad);
    ExportedTupleStruct(tup.0,tup.1)
}

#[wasm_bindgen]
pub fn test() -> ExportedTupleStruct {
    let tup = fibonacci_search(10,-10.0,10.0,|x|x.powf(2.0));
    ExportedTupleStruct(tup.0,tup.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn norm_test() {
        assert_eq!(norm(&(1.0,1.0),&(2.0,2.0)), (2.0_f32).sqrt());
        // assert(norm(&()) > 0.001);
    }
    #[test]
    fn fibonacci_test() {
        assert_eq!(fib(10), 55);
    }
    #[test]
    fn test_fibonacci_search() {
        assert_eq!(fibonacci_search(10,-10.0,10.0,|x|x.powf(2.0)), (0.0,0.0))
    }
    #[test]
    fn test_geradengleichung() {
        assert_eq!(g(2.0,2.0,grad)(1.0),1.0 );
        assert_eq!(g(2.0,2.0,grad)(10.0),10.0 );
    }
    #[test]
    fn test_gradiententestverfahren() {
        assert_eq!(gradientenanstiegsverfahren(2.0,2.0,f,grad), (1.0,1.0));
    }
}
