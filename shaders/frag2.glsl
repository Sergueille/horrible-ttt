#version 330 core

out vec4 color;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

void main() {
    color = vec4(local_pos_i.gbr, 1.0);
}

