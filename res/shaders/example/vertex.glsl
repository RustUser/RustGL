#version 330 core

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

layout (location = 0) in vec3 Position;

void main()
{
    gl_Position = projection * view * model * vec4(Position, 1.0);
}