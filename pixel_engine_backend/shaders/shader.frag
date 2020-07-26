#version 450

// Changed
layout(location=0) in vec3 v_tex_coords;
layout(location=0) out vec4 f_color;

// New
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    // Changed
    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords.xy  / v_tex_coords.z);
}
