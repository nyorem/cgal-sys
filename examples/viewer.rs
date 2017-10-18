extern crate cgal_sys;
#[macro_use] extern crate glium;

use cgal_sys::{Triangle, Vec2};
use cgal_sys::{convex_hull_2, delaunay_2};

use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowEvent, MouseButton};
use glium::Surface;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const N_POINTS_MAX : usize = 100;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            position: [x, y],
        }
    }
}

fn to_opengl_frame(x: f32, y: f32) -> (f32, f32) {
    let x = 2.0 * x / (WINDOW_WIDTH as f32) - 1.0;
    let y = 2.0 * y / (WINDOW_HEIGHT as f32) - 1.0;
    (x, -y)
}

fn main() {
    // Setup
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("Viewer");
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Programs
    // Points
    let point_vs = r#"
        #version 330 core

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0f, 1.0f);
        }
    "#;

    let points_gs = r#"
        #version 330 core

        layout (points) in;
        layout (line_strip, max_vertices = 5) out;

        #define LINE_HALF_SIZE (0.03f)

        void main() {
            gl_Position = gl_in[0].gl_Position + vec4(-LINE_HALF_SIZE, 0.0f, 0.0f, 0.0f);
            EmitVertex();

            gl_Position = gl_in[0].gl_Position + vec4( LINE_HALF_SIZE, 0.0f, 0.0f, 0.0f);
            EmitVertex();

            gl_Position = gl_in[0].gl_Position;
            EmitVertex();

            gl_Position = gl_in[0].gl_Position + vec4(0.0f, -LINE_HALF_SIZE, 0.0f, 0.0f);
            EmitVertex();

            gl_Position = gl_in[0].gl_Position + vec4(0.0f,  LINE_HALF_SIZE, 0.0f, 0.0f);
            EmitVertex();

            EndPrimitive();
        }
    "#;

    let points_fs = r#"
        #version 330 core

        uniform vec4 color;

        out vec4 f_color;

        void main() {
            f_color = color;
        }
    "#;

    let points_program = glium::Program::from_source(&display,
                                                     &point_vs, &points_fs, Some(&points_gs)).unwrap();
    let triangles_program = glium::Program::from_source(&display, &point_vs, &points_fs, None).unwrap();

    let mut points_buffer = glium::VertexBuffer::empty_dynamic(&display, N_POINTS_MAX).unwrap();
    let points_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let mut closed = false;
    let mut cursor_position: Option<(f32, f32)> = None; // in OpenGL frame
    let mut point_idx = 0;

    let mut points_vec2 = Vec::new();
    let mut delaunay_tri: Option<Vec<Triangle>> = None;
    let mut chull: Option<Vec<i32>> = None; // TODO

    let wireframe_params = glium::DrawParameters {
        polygon_mode : glium::draw_parameters::PolygonMode::Line,
        ..Default::default()
    };

    while !closed {
        let mut target = display.draw();
        target.clear_color(0.2, 0.3, 0.3, 1.0);

        // Draw
        // Points
        let points_uniforms = uniform! {
            color: [ 1.0, 0.0, 0.0, 1.0f32 ],
        };

        target.draw(points_buffer.slice(0 .. point_idx).unwrap(),
                    &points_indices, &points_program, &points_uniforms,
                    &Default::default()).unwrap();

        // Delaunay triangulation
        let triangles_uniforms = uniform! {
            color: [ 0.0, 0.0, 1.0, 1.0f32 ],
        };

        if let Some(triangles) = delaunay_tri.as_ref() {
            // TODO: use iter
            let mut triangles_gl = Vec::new();
            for t in triangles {
                triangles_gl.push(t.0 as u32);
                triangles_gl.push(t.1 as u32);
                triangles_gl.push(t.2 as u32);
            }

            // TODO: do not recreate index buffer each frame
            let triangles_indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                                            &triangles_gl).unwrap();

            target.draw(points_buffer.slice(0 .. point_idx).unwrap(),
                        &triangles_indices, &triangles_program, &triangles_uniforms,
                        &wireframe_params).unwrap();
        }

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => {
                        closed = true;
                    },

                    WindowEvent::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Escape) => {
                                    closed = true;
                                },

                                Some(VirtualKeyCode::C) => {
                                    println!("clear");
                                    delaunay_tri = None;
                                    chull = None;
                                    point_idx = 0;
                                    points_vec2.clear();
                                },

                                Some(VirtualKeyCode::H) => {
                                    println!("convex hull");
                                    chull = Some(convex_hull_2(&points_vec2));
                                },

                                Some(VirtualKeyCode::T) => {
                                    println!("delaunay");
                                    delaunay_tri = Some(delaunay_2(&points_vec2));
                                },

                                _ => (),
                            }
                        }
                    },

                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            if let Some((x, y)) = cursor_position {
                                points_vec2.push(Vec2::new(x, y));
                                points_buffer.map()[point_idx] = Vertex::new(x, y);
                                point_idx += 1;
                                point_idx %= N_POINTS_MAX;
                            }
                        }
                    },

                    WindowEvent::MouseMoved { position: (x,y), .. } => {
                        cursor_position = Some(to_opengl_frame(x as f32, y as f32));
                    },

                    _ => (),
                },

                _ => (),
            }
        });
    }
}
