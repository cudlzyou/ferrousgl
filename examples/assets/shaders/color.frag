#version 330 core

out vec4 FragColor;

in vec3 pos;

void main() {
    vec3 bottomColor = vec3(0.0, 0.2, 0.0);
    vec3 centerColor = vec3(0.3, 0.6, 0.1);
    vec3 topColor = vec3(0.45, 0.6, 0.4);

    float bottomCenterFactor = clamp((pos.y/4.0), 0.0, 1.0);
    vec3 bottomCenter = mix(bottomColor, centerColor, bottomCenterFactor);

    float topFactor = clamp((pos.y/4.0)-1.0, 0.0, 1.0);
    vec3 color = mix(bottomCenter, topColor, topFactor);

    FragColor = vec4(color, 1.0);
}
