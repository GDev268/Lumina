#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec3 FragPos;

//64
layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
} ubo;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;


void main() {
  gl_Position = ubo.projectionViewMatrix * push.modelMatrix * vec4(position, 1.0);

  FragPos = vec3(push.modelMatrix * vec4(position,1.0));
}