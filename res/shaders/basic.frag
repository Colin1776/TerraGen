#version 330 core

precision mediump float;

in vec2 tex_coord;

out vec4 color;

uniform sampler2D tex;

void main()
{
    color = texture(tex, vec2(tex_coord.x / 16.0, tex_coord.y / 16.0));
    // color = vec4(1.0, 1.0, 1.0, 1.0);
}
