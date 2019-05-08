extern crate sdl2;
extern crate gl;

pub mod render_gl;

const WIDTH: i32 = 800; // for some reason the compiler won't accept u16 or u32 so i guess we are wasting ram now :(
const HEIGHT: i32 = 600; // gotta save where you can. especially when you have already shit code.

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();

    //set up gl context attribs
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    // create window
    let window = video_subsystem.window("Test OpenGl", WIDTH as u32, HEIGHT as u32).resizable().opengl().build().unwrap();

    // Setup gl.
    let mut event_pump = _sdl.event_pump().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
    unsafe {
        gl::Viewport(0, 0, WIDTH, HEIGHT); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::ffi::CString;

    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    'main: loop {
        // TODO: Make a better handler?
        for _event in event_pump.poll_iter() {
            match _event {
                sdl2::event::Event::Quit {..} => break 'main,_ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }



        window.gl_swap_window();
    }

}