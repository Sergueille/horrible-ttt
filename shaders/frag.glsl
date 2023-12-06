#version 330 core

out vec4 color;
in vec3 pos_i;

void main() {
    color = vec4(pos_i + vec3(0.5, 0.5, 0.0), 1.0);
}

