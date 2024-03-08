#version 330 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_normal;
layout (location = 2) in vec2 in_tex_coord;

out vec3 position;

void main()
{
    position = in_position;
    gl_Position = vec4(in_position, 1.0);
}
