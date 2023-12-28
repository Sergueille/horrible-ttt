#version 330 core

in vec3 pos;
in vec2 uv;
out vec3 pos_i;
out vec3 local_pos_i;
out vec2 uv_i;

uniform mat4 transform;
uniform mat4 projection;

void main() {
    vec4 res = projection * transform * vec4(pos, 1.0);
    gl_Position = res;
    pos_i = res.xyz;
    local_pos_i = pos;
    uv_i = uv;
}
