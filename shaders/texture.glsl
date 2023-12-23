#version 330 core

out vec4 color_o;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform sampler2D tex;
uniform vec4 color;

void main() {
    color_o = texture(tex, uv_i) * color;
    if (color_o.a < 0.1) discard;
}

