#version 330 core

out vec4 color_o;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform vec4 color;

void main() {
    color_o = color;
}

