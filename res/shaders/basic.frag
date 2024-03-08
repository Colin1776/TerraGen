#version 330 core

precision mediump float;

in vec3 position;
out vec4 color;

uniform float blue;

void main()
{
    color = vec4(1.0, 1.0, blue, 1.0);
}
