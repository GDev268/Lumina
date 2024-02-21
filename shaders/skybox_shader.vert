#version 450
layout(location = 0) in vec3 position;
layout(location = 0) out vec3 FragUV;

//64
layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
} ubo;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;

void main() {
    FragUV = position;
    vec4 pos = ubo.projectionViewMatrix * push.modelMatrix * vec4(position, 1.0);
    gl_Position = pos;
}