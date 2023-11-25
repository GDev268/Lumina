#version 450

layout(location = 0) in vec3 position;

layout(push_constant) uniform Push_Vertex {
  mat4 modelMatrix;
  mat4 viewMatrix;
  mat4 projectionMatrix;
} push;

void main() {
  gl_Position = push.projectionMatrix * push.viewMatrix * push.modelMatrix * vec4(position,1.0);
}


