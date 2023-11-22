#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec3 Normal;

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;    
  float shininess;
}; 

struct Light {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

layout(set = 0,binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
  vec3 directionToLight;
} ubo;

layout(set = 0, binding = 1) uniform ObjectProperties {
  vec3 viewPos;
  vec3 ambient;
} object;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;


const float AMBIENT = 0.00000000000000000000001;

void main() {
  gl_Position = ubo.projectionViewMatrix * push.modelMatrix * vec4(position, 1.0);

  vec3 normalWorldSpace = normalize(mat3(push.normalMatrix) * normal);

  float lightIntensity = AMBIENT + max(dot(normalWorldSpace, ubo.directionToLight), 0);

  fragColor = object.ambient;
}