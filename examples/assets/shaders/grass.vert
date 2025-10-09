#version 330 core

layout(location = 0) in vec3 aPosition;
layout(location = 1) in mat4 aInstanceTransform;
layout(location = 5) in vec4 aInstanceColor; // next slot after mat4

uniform mat4 transform;
uniform float time; // new uniform for animation

out vec3 vLocalPos;
out vec4 vInstanceColor;

void main()
{
    vLocalPos = aPosition;
    vInstanceColor = aInstanceColor;

    // Simple wind sway
    float swayX = sin(aPosition.x * 2.0 + time * 3.0) * 0.16;
    float swayZ = cos(aPosition.z * 2.0 + time * 2.0) * 0.16;

    vec3 animatedPos = aPosition + vec3(swayX, 0.0, swayZ);

    gl_Position = transform * aInstanceTransform * vec4(animatedPos, 1.0);
}
