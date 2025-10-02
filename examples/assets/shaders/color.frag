#version 330 core

out vec4 FragColor;

uniform vec3 colorOne;

void main()
{
    FragColor = vec4(colorOne, 1.0);
}
