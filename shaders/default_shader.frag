#version 450

layout (location = 0) in vec3 FragColor;

layout (location = 0) out vec4 outColor;

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

layout(set = 0,binding = 1) ObjectProperties {
  vec3 viewPos,
  Material material,
  Light cur_light
}

layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
  vec3 directionToLight;
} ubo;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;


void main() {
  outColor = vec4(FragColor, 1.0);
}
