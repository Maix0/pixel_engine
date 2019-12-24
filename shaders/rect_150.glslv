#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;
in vec3 a_Color;
out vec4 v_Color;
out vec2 v_Uv;

void main() {
    v_Color = vec4(a_Color, 1.0);
    v_Uv = a_Uv;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
