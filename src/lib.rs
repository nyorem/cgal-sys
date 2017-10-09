extern crate libc;
use libc::{c_int, c_void, size_t};

/// A 2d point.
#[repr(C)]
#[derive(Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}


impl Vec2 {
    pub fn new (x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

/// A triangle represented by 3 indices.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Triangle(i32, i32, i32);

extern {
    fn c_convex_hull_2(arr: *const Vec2,
                       n_points: size_t,
                       size_hull: *mut size_t) -> *const c_int;

    fn c_delaunay_2(arr: *const Vec2,
                    n_points: size_t,
                    size_tri: *mut size_t) -> *const c_int;
}

/// Computes the convex hull of the input points.
pub fn convex_hull_2 (arr: &[Vec2]) -> Vec<i32> {
    let mut size_chull: usize = 0;
    let size_chull_ptr = &mut size_chull as *mut usize;

    unsafe {
        let chull_ptr = c_convex_hull_2(arr.as_ptr(), arr.len(), size_chull_ptr);
        let chull_slice =  std::slice::from_raw_parts(chull_ptr, size_chull) ;
        libc::free(chull_ptr as *mut c_void);
        chull_slice.to_vec()
    }
}

/// Computes the Delaunay triangulation of the input points.
pub fn delaunay_2 (arr: &[Vec2]) -> Vec<Triangle> {
    let mut size_tri: usize = 0;
    let size_tri_ptr = &mut size_tri as *mut usize;

    unsafe {
        let tri_ptr = c_delaunay_2(arr.as_ptr(), arr.len(), size_tri_ptr);
        let tri_slice =  std::slice::from_raw_parts(tri_ptr, 3 * size_tri) ;
        libc::free(tri_ptr as *mut c_void);
        let triangles = tri_slice.to_vec();

        let mut dt: Vec<Triangle> = Vec::new();
        for t in triangles.chunks(3) {
            assert_eq!(t.len(), 3);
            dt.push(Triangle(t[0], t[1], t[2]));
        }
        dt
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

    #[test]
    fn test_delaunay_2() {
        let points = vec![Vec2::new(1.0, 1.0),
                          Vec2::new(0.0, 0.0),
                          Vec2::new(1.0, 0.0),
                          Vec2::new(0.0, 1.0)
                         ];

        let dt = delaunay_2(points.as_slice());

        assert_eq!(dt, vec![Triangle(3, 2, 0),
                            Triangle(3, 1, 2)
                           ]);
    }
}
