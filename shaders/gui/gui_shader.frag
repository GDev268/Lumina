#version 450
layout(location = 0) in vec4 Color;
layout(location = 1) in vec2 FragUV;

layout (location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1.0,1.0,1.0,1.0);
}