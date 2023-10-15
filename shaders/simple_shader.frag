#version 450

layout (location = 0) in vec3 fragColor;

layout (location = 0) out vec4 outColor;

struct Lime {
  mat4 test1;
  mat4x2 test2;
};

layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
  vec3 directionToLight;
  Lime three;
} ubo;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;

void main() {
  outColor = vec4(fragColor, 1.0);
}
