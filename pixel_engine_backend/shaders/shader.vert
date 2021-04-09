#version 450

layout(location=0) in vec3 a_position;
// Changed
layout(location=1) in vec3 a_tex_coords;
layout(location=2) in vec4 a_tint;
// Changed
layout(location=0) out vec3 v_tex_coords;
layout(location=1) out vec4 f_tint;
void main() {
    // Changed
    v_tex_coords = a_tex_coords;
    f_tint = a_tint;
    
	gl_Position = vec4(a_position, 1.0);
}
