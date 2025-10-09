#version 330 core

in vec3 vLocalPos;
in vec4 vInstanceColor;

out vec4 FragColor;

void main()
{
    // example: multiply color by height gradient
    float heightTint = clamp(0.1 + vLocalPos.y * 0.1, 0.0, 1.0);
    FragColor = vec4(vInstanceColor.rgb * heightTint, 1.0);
}
