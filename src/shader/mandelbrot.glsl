#version 330 core

uniform ivec2 viewport;
uniform float zoom;
uniform vec2 center;
uniform int quality;

vec2 complex_square(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

out vec4 outColor;

void main()
{
    int iterations = int(64.0 * max(log(zoom), 1.0) * float(quality));
    vec2 c = vec2((2.0 * gl_FragCoord.x / float(viewport.x) - 1.0) / zoom + center.x, (2.0 * gl_FragCoord.y / float(viewport.y) - 1.0) / zoom + center.y);
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
