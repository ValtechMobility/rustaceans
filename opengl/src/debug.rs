pub unsafe fn check_gl_error(location: &str) {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
        println!("OpenGL error at {}: {}", location, error);
    }
}
