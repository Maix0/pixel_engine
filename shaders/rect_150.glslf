#version 150 core

uniform sampler2D t_Awesome;

in vec4 v_Color;
in vec2 v_Uv;
out vec4 Target0;

void main() {
    Target0 = texture(t_Awesome, v_Uv);
	
}
