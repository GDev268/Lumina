#version 450 
layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec4 Color;
layout(location = 1) out vec2 FragUV;

//64
layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
} ubo;

void main() {
    gl_Position = vec4(position,0.0, 1.0);

    Color = color;
    FragUV = uv;
}