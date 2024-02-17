#version 450

layout(location = 0) in vec2 fragTexCoord;

layout(binding = 0) uniform sampler2D imageTexture;

//12
layout(set = 0, binding = 1) uniform TestUwU {
  vec3 testuwu;
} ubo;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(ubo.testuwu,1.0);
}