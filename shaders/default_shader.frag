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
layout(set = 0,binding = 1) uniform TestUBO {
  Material material;
  vec3 viewPos;
} object ;

layout(set = 0,binding = 2) uniform sampler2D normalMap;

void main() {

  outColor = texture(normalMap,FragUV) * vec4(object.material.ambient,1.0);
}



  /*vec3 ambient = object.cur_light.ambient * object.material.ambient;

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

  vec3 result = ambient + diffuse + specular;
  outColor = vec4(result,1.0);*/
