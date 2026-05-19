#version 330 core

uniform ivec2 viewport;
uniform float zoom;
uniform vec2 center;
uniform int iterations;
uniform vec2 parameter;

vec2 complex_square(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

out vec4 outColor;

void main()
{
    vec2 z = vec2((2.0 * gl_FragCoord.x / float(viewport.x) - 1.0) / zoom + center.x, (2.0 * gl_FragCoord.y / float(viewport.y) - 1.0) / zoom + center.y);
    for (int i = 0; i < iterations; i++) {
        z = complex_square(z) + parameter;
        if (dot(z, z) > 4) {
            float convergence = float(i) / float(iterations);
            outColor = vec4(0.0, convergence, convergence, 1.0f);
            return;
        }
    }
    outColor = vec4(1.0, 1.0, 1.0, 1.0f);
}
