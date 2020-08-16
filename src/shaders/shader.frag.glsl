#version 450

layout(location = 0) in vec2 texCoord;

/*layout(set = 1, binding = 1) uniform Uniform{
    vec4 blend;
};*/

layout(set = 0, binding = 0) uniform texture2D texColor;
layout(set = 0, binding = 1) uniform sampler sample_name;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = texture(sampler2D(texColor, sample_name), texCoord);
    //outColor = vec4(rgb, 1.0) * blend;
}