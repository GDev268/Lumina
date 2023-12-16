#version 450
#define MAX_LIGHTS = 99;

layout (location = 0) in vec3 FragPos;
layout (location = 1) in vec3 Normal;
layout(location = 2) in vec2  FragUV;

layout (location = 0) out vec4 outColor;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;

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

//64
layout(set = 0,binding = 1) uniform MaterialInfo {
  Material material;
  vec3 viewPos;
} object;

//64
layout(set = 0,binding = 2) uniform LightInfo {
  Light light;
} object_light;

layout(set = 0,binding = 3) uniform sampler2D normalMap;

void main() {
  vec3 ambient = object_light.light.ambient * texture(normalMap,FragUV).rgb;

  //Diffuse
  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(object_light.light.position - FragPos);
  float diffuseDistance = max(dot(normal,lightDirection),0.0);
  vec3 diffuse = object_light.light.diffuse * (diffuseDistance * object.material.diffuse);

  //Specular
  vec3 viewDirection = normalize(object.viewPos - FragPos);
  vec3 reflectDirection = reflect(-object_light.light.position,normal);
  float spec = pow(max(dot(viewDirection,reflectDirection),0.0), object.material.shininess);
  vec3 specular = object_light.light.specular * (spec * object.material.specular);

  vec3 result = ambient + diffuse + specular;

  outColor = vec4(result,1.0);
}
