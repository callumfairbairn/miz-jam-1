#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coord;

layout(set = 1, binding = 0) uniform Uniform{
    vec2 transform;
};

layout(location = 0) out vec2 texCoordOut;

void main() {
    gl_Position = vec4(position + transform, 0.0, 1.0);
    //gl_Position = vec4(position, 0.0, 1.0);

    texCoordOut = tex_coord;
}