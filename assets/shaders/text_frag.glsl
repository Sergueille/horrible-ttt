#version 330 core

out vec4 color_o;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform sampler2D tex;
uniform vec4 color;

void main() {
    color_o = texture(tex, uv_i) * color;

    float alpha = (color_o.r - 0.21) * 50.0;

    if (alpha < 0.0) discard;
    color_o = vec4(color.rgb, alpha);
}

