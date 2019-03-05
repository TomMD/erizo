use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use gl::types::*;
use glutin::dpi::*;
use glutin::GlContext;
use failure::Error;

mod stl;
mod mesh;
use crate::stl::load_from_file;

////////////////////////////////////////////////////////////////////////////////

// Shader sources
static VS_SRC: &'static str = "
#version 330
layout(location=0) in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 330
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

macro_rules! gl_check {
    ($item:ident, $status:ident, $get_iv:ident, $get_log:ident) => {

        let mut status = gl::FALSE as GLint;
        gl::$get_iv($item, gl::$status, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::$get_iv($item, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = vec![0; len as usize];
            gl::$get_log($item, len, ptr::null_mut(),
                         buf.as_mut_ptr() as *mut GLchar);

            panic!("{}", str::from_utf8(&buf)
                .expect("Log info not valid utf8"));
        }
    }
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        gl_check!(shader, COMPILE_STATUS, GetShaderiv, GetShaderInfoLog);
        shader
    }
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        gl_check!(program, LINK_STATUS, GetProgramiv, GetProgramInfoLog);
        program
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Error> {
    //let mesh = load_from_file("/Users/mkeeter/Models/porsche.stl")?;
    let mesh = load_from_file("sphere.stl")?;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vert_vbo = 0;
    let mut tri_vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a VBO for the mesh vertices
        gl::GenBuffers(1, &mut vert_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vert_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mesh.vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
            mem::transmute(&mesh.vertices[0]),
            gl::STATIC_DRAW,
        );

        // Then, create a VBO for the triangle indices
        gl::GenBuffers(1, &mut tri_vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, tri_vbo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mesh.triangles.len() * mem::size_of::<u32>()) as GLsizeiptr,
            mem::transmute(&mesh.triangles[0]),
            gl::STATIC_DRAW,
        );

        // Specify the layout of the vertex data
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, /* Vertex attribute location */
            3, /* Number of elements */
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0, /* Offset */
            ptr::null(),
        );
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::UseProgram(program);
            gl::DrawElements(gl::TRIANGLES, mesh.triangles.len() as i32,
                             gl::UNSIGNED_INT, ptr::null());
        }

        gl_window.swap_buffers().unwrap();
    }

    Ok(())
}
