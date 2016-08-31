extern crate kiss3d;
extern crate nalgebra as na;
extern crate gl;


use std::rc::Rc;
use std::cell::RefCell;
use na::{Point3, Vector3};
use kiss3d::window::Window;
use kiss3d::resource::{Mesh, MeshManager};
use kiss3d::light::Light;
use gl::types::{GLfloat, GLuint};

static LENGTH: GLfloat = 21.5 / 21.5; // [cm]
static WIDTH:  GLfloat = 3.8 / 21.5;
static HEIGHT: GLfloat = 0.8 / 21.5;

static PILLAR_HEIGHT: GLfloat = 6.6 / 21.5;
static PILLAR_WIDTH:  GLfloat = 9.55 / 21.5;
static PILLAR_DEPTH:  GLfloat = 3.05 / 21.5;

// outer top -> outer bottom -> inner bottom -> inner top
// と与えられた頂点たちに対してリング状にメッシュを返す。
fn gen_mesh(n: usize, vertices: Vec<Point3<GLfloat>>) -> Rc<RefCell<Mesh>> {
    let mut indices: Vec<Point3<GLuint>> = Vec::with_capacity(8 * n);
    for i in 0..(n-1) {
        let a0 = (4 * i + 0) as GLuint;
        let a1 = (4 * i + 1) as GLuint;
        let a2 = (4 * i + 2) as GLuint;
        let a3 = (4 * i + 3) as GLuint;
        let b0 = (4 * i + 4) as GLuint;
        let b1 = (4 * i + 5) as GLuint;
        let b2 = (4 * i + 6) as GLuint;
        let b3 = (4 * i + 7) as GLuint;
        
        indices.push(Point3::new(a3, a0, b3)); // top
        indices.push(Point3::new(b3, a0, b0));
                                         
        indices.push(Point3::new(b1, a1, b2)); // bottom
        indices.push(Point3::new(b2, a1, a2));
                                         
        indices.push(Point3::new(a2, a3, b2)); // inner
        indices.push(Point3::new(b2, a3, b3));
                                         
        indices.push(Point3::new(b0, a0, b1)); // outer
        indices.push(Point3::new(b1, a0, a1));
    }

    Rc::new(RefCell::new(Mesh::new(vertices, indices, None, None, false)))
}


fn gen_curve_points(n: usize) -> Vec<Point3<GLfloat>> {
    let mut vertices: Vec<Point3<GLfloat>> = Vec::with_capacity(4 * (n + 1));
    for i in 0..(n+1) {
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / (n as f32) / 8.0;
        let x = theta.cos();
        let y = theta.sin();

        let ro = LENGTH + WIDTH / 2.0;
        let ri = LENGTH - WIDTH / 2.0;
        
        vertices.push(Point3::new(ro * x - LENGTH, ro * y,  HEIGHT/2.0)); // outer top
        vertices.push(Point3::new(ro * x - LENGTH, ro * y, -HEIGHT/2.0)); // outer bottom
        vertices.push(Point3::new(ri * x - LENGTH, ri * y, -HEIGHT/2.0)); // inner bottom
        vertices.push(Point3::new(ri * x - LENGTH, ri * y,  HEIGHT/2.0)); // inner top
    }
    vertices
}

fn gen_straight_points(n: usize) -> Vec<Point3<GLfloat>> {
    let mut vertices: Vec<Point3<GLfloat>> = Vec::with_capacity(4 * (n + 1));
    for i in 0..(n+1) {
        let theta = (i as f32) / (n as f32);
        let y = theta * LENGTH;

        let ro =  WIDTH / 2.0;
        let ri = -WIDTH / 2.0;
        
        vertices.push(Point3::new(ro, y,  HEIGHT/2.0)); // outer top
        vertices.push(Point3::new(ro, y, -HEIGHT/2.0)); // outer bottom
        vertices.push(Point3::new(ri, y, -HEIGHT/2.0)); // inner bottom
        vertices.push(Point3::new(ri, y,  HEIGHT/2.0)); // inner top
    }
    vertices
}

fn main() {
    let n = 16;

    // デフォルトのカメラが反対から覗き込むことになってて激おこなので。
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0.0f32, 0.0, 1.0), na::origin());
    camera.set_dist(4.0);
    
    let mut window = Window::new("Kiss3d: custom_mesh");
    window.set_light(Light::StickToCamera);

    MeshManager::get_global_manager(|mm| mm.add(gen_mesh(n, gen_curve_points(n)), "curve"));
    MeshManager::get_global_manager(|mm| mm.add(gen_mesh(n, gen_straight_points(n)), "straight"));
    
    let mut c1 = window.add_geom_with_name("curve", na::one()).unwrap();
    let mut c2 = window.add_geom_with_name("straight", na::one()).unwrap();
    let mut c3 = window.add_geom_with_name("curve", na::one()).unwrap();

    
    let pi2 = std::f32::consts::FRAC_PI_2;
    c1.prepend_to_local_rotation(&Vector3::new(0.0, 0.0, -pi2));
    c1.append_translation(&Vector3::new(1.0, 0.0, 0.0));

    c3.prepend_to_local_rotation(&Vector3::new(0.0, 0.0, -pi2/2.0));
    c3.append_translation(&Vector3::new(1.0f32 + 0.5f32.sqrt(), 1.0 - 0.5f32.sqrt(), 0.0));
    
    c2.append_rotation(&Vector3::new(0.0, 0.0, -pi2));

    c1.set_color(0.0, 0.4, 1.0);
    c2.set_color(0.0, 0.4, 1.0);
    c3.set_color(0.0, 0.4, 1.0);
    
    while window.render_with_camera(&mut camera) {

    }
}
