#version 330 core

out vec4 color;
in vec3 pos_i;
in vec3 local_pos_i;
in vec2 uv_i;

uniform float ratio;

// OPTI: berk
void main() {
    if (uv_i.x < 0.5 / ratio || uv_i.x > 1.0 - 0.5 / ratio)
    {
        vec2 coordA = uv_i - vec2(0.5 / ratio, 0.5);
        vec2 coordB = uv_i - vec2(1.0 - 0.5 / ratio, 0.5);
        coordA.x *= ratio;
        coordB.x *= ratio;
        float dist = min(dot(coordA, coordA), dot(coordB, coordB));
        if (dist > 0.25) discard;
    }

    color = vec4(1.0, 0.0, 1.0, 1.0);
}

