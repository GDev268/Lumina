#version 450

layout(location = 0) in vec3 position;

//64
layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
} ubo;

void main() {
  gl_Position = ubo.projectionViewMatrix * vec4(position, 1.0);
}