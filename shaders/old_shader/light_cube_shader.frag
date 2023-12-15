#version 450

layout(location = 0) out vec4 outColor;

layout(push_constant) uniform Push_Fragment {
  vec3 objectColor;
  vec3 lightColor;
} push;

void main() {
  outColor = vec4(push.objectColor * push.lightColor,1.0);
}
