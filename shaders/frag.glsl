#version 330 core

out vec4 color;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

void main() {
    float val = dot(normalize(local_pos_i), vec3(1.0, 0.0, 0.0));
    color = vec4(val, val, val, 1.0);
}

