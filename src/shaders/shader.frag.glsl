#version 450

layout(location = 0) in vec2 texCoord;

layout(set = 1, binding = 1) uniform Uniform{
    vec4 blend;
    vec4 override;
};

layout(set = 0, binding = 0) uniform texture2D texColor;
layout(set = 0, binding = 1) uniform sampler sample_name;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 tex = texture(sampler2D(texColor, sample_name), texCoord);
    if (tex.a > 0.0 && override.a > 0.0) {
        outColor = override;
    } else {
        outColor = tex * blend;
    }
}