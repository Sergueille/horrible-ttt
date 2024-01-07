#version 330 core

out vec4 color_o;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform sampler2D tex;
uniform vec4 color;

void main() {
    vec3 res = texture(tex, uv_i * color.a).rgb * color.rgb;
    color_o = vec4(res, 1.0);
}

