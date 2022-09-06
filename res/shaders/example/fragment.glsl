#version 330 core

out vec4 Color;
uniform vec4 color;

void main()
{
    Color = vec4(1.0f, 0.5f, 0.0f, 1.0f) * color;
}