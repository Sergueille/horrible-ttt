#version 330 core

in vec3 pos;
out vec3 pos_i;

void main() {
    gl_Position = vec4(pos, 1.0);
    pos_i = pos;
}
