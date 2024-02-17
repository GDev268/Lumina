#version 450

layout (location = 0) out vec4 outColor;

//12
layout(set = 0, binding = 1) uniform RecorderColor {
  vec3 color;
} rec;

void main() {
    outColor = vec4(rec.color,1.0);
}