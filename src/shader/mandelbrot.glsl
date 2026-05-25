#version 330 core

uniform vec2 scaling;
uniform vec2 offset;
uniform int iterations;

vec2 complex_square(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

out vec4 outColor;

void main()
{
    vec2 c = gl_FragCoord.xy * scaling + offset;
    vec2 z = vec2(0.0f, 0.0f);
    for (int i = 0; i < iterations; i++) {
        z = complex_square(z) + c;
        if (dot(z, z) > 4.0f) {
            outColor = vec4(float(i) / float(iterations), 0.0, 0.0, 1.0f);
            return;
        }
    }
    outColor = vec4(1.0, 1.0, 1.0, 1.0f);
}
