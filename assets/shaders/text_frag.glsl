#version 330 core

out vec4 color_o;
in vec2 uv_i;
in vec4 color_i;

uniform sampler2D tex;

void main() {
    color_o = texture2D(tex, uv_i);

    float alpha = (color_o.r - 0.045) * 300.0;

    if (alpha < 0.0) discard;
    color_o = vec4(color_i.rgb, alpha * color_i.a);
}

