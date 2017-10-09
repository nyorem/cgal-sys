extern crate libc;
use libc::{c_int, size_t};

#[repr(C)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new (x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

extern {
    fn c_convex_hull_2(arr: *const Vec2, n_points: size_t, size_hull: *mut size_t) -> *const c_int;
}

pub fn convex_hull_2 (arr: &[Vec2]) -> Vec<i32> {
    let mut size_chull: usize = 0;
    let size_chull_ptr = &mut size_chull as *mut usize;

    unsafe {
        let chull_ptr = c_convex_hull_2(arr.as_ptr(), arr.len(), size_chull_ptr);
        let chull_slice =  std::slice::from_raw_parts(chull_ptr, size_chull) ;
        chull_slice.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_2() {
        let points = vec![Vec2::new(1.0, 1.0),
                          Vec2::new(0.0, 0.0),
                          Vec2::new(1.0, 0.0),
                          Vec2::new(0.5, 0.5),
                          Vec2::new(0.0, 1.0)
                         ];
        let chull = convex_hull_2(points.as_slice());
        assert_eq!(chull, vec![1, 2, 0, 4]);
    }
}
