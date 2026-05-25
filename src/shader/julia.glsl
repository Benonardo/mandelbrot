#version 330 core

uniform vec2 scaling;
uniform vec2 offset;
uniform int iterations;
uniform vec2 parameter;

vec2 complex_square(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

out vec4 outColor;

void main()
{
    vec2 z = gl_FragCoord.xy * scaling + offset;
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
