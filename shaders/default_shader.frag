#version 450

layout (location = 0) in vec3 FragPos;
layout (location = 1) in vec3 Normal;

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


layout(set = 0, binding = 0) uniform GlobalUBO {
  mat4 projectionViewMatrix;
  vec3 directionToLight;
} ubo;

layout(set = 0, binding = 1) uniform ObjectProperties {
  vec3 viewPos;
  Material material;
  Light cur_light;
} object;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;


void main() {
  /*//Ambient
  vec3 ambient = object.cur_light.ambient * object.material.ambient;

  //Diffuse
  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(object.cur_light.position - FragPos);
  float diffuseDistance = max(dot(normal,lightDirection),0.0);
  vec3 diffuse = object.cur_light.diffuse * (diffuseDistance * object.material.diffuse);

  //Specular
  vec3 viewDirection = normalize(object.viewPos - FragPos);
  vec3 reflectDirection = reflect(-object.cur_light.position,normal);
  float spec = pow(max(dot(viewDirection,reflectDirection),0.0), object.material.shininess);
  vec3 specular = object.cur_light.specular * (spec * object.material.specular);

  vec3 result = ambient + diffuse + specular;*/

  outColor = vec4(object.cur_light.ambient * object.material.ambient, 1.0);
}
