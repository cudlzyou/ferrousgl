#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 3) in vec3 instanceOffset;
layout(location = 4) in mat4 instanceRotation;
layout(location = 6) in float instanceBend;

uniform mat4 transform;
uniform float time;

out vec3 pos;
out vec3 normal;

void main() {
    vec3 bentPos = aPos + vec3(0.0, 0.0, (aPos.y * aPos.y) * 0.05 * instanceBend);
    vec3 worldPos = (instanceRotation * vec4(bentPos, 1.0)).xyz + instanceOffset;

    float sway = sin(time * 0.008 + (instanceOffset.x * 0.1 + instanceOffset.z * 0.1))
    * 0.5 * aPos.y;
    worldPos += vec3(sway, 0.0, 0.0); // sway along global +X

    gl_Position = transform * vec4(worldPos, 1.0);

    pos = aPos;
}
