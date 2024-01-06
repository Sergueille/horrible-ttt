#version 330 core

in vec4 pos_attr;
in vec4 uv_attr;
in vec4 color_attr;
in vec3 pos;
out vec2 uv_i;
out vec4 color_i;

void main() {
    gl_Position = vec4(
        (pos_attr.x + pos_attr.z * pos.x) * 2.0 - 1.0,
        (pos_attr.y + pos_attr.w * pos.y) * 2.0 - 1.0, 
        0.0, 1.0);

    uv_i = vec2(
        uv_attr.x + uv_attr.z * pos.x,
        uv_attr.y + uv_attr.w * pos.y
    );

    color_i = color_attr;
}
