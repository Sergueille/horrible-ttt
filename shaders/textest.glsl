#version 330 core

out vec4 color;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform sampler2D tex;

void main() {
    color = texture(tex, uv_i);
    if (color.a < 0.5) discard;
}

