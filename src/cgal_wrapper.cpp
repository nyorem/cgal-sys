#include <CGAL/Exact_predicates_inexact_constructions_kernel.h>
#include <CGAL/convex_hull_2.h>

#include <vector>
#include <map>

// 2D point
typedef struct {
    float x;
    float y;
} vec2;

typedef CGAL::Exact_predicates_inexact_constructions_kernel Kernel;
typedef CGAL::Point_2<Kernel> Point_2;

extern "C" int* c_convex_hull_2 (vec2 *points, int n_points, int* size_hull) {
    std::vector<Point_2> points_vec;
    std::map<Point_2, int> indices_map;
    for (int i = 0; i < n_points; ++i) {
        Point_2 p(points[i].x, points[i].y);
        points_vec.push_back(p);
        indices_map[p] = i;
    }

    std::vector<Point_2> hull;
    CGAL::convex_hull_2(points_vec.begin(), points_vec.end(), std::back_inserter(hull));

    *size_hull = hull.size();
    // TODO: free
    int* points_hull = new int[hull.size()];

    for (std::size_t i = 0; i < hull.size(); ++i) {
        points_hull[i] = indices_map[hull[i]];
    }

    return points_hull;
}

