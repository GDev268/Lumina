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

struct DirectionalLight {
  vec3 direction;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct PointLight {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

struct SpotLight {
  vec3 position;
  vec3 direction;

  float cut_off;
  float outer_cut_off;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};


struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;    
  float shininess;
}; 

//64
layout(set = 0,binding = 1) uniform MaterialInfo {
  Material material;
  vec3 viewPos;
} object;

//128
layout(set = 0,binding = 2) uniform LightInfo {
  SpotLight light;
} object_light;

layout(set = 0,binding = 3) uniform sampler2D normalMap;

void main() {
  //OlD ONE
  /*vec3 ambient = object_light.light.ambient * texture(normalMap,FragUV).rgb;

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

  vec3 result = ambient + diffuse + specular;*/

  //Directional Light
  /*vec3 ambient = object_light.light.ambient * texture(normalMap,FragUV).rgb;

  //Diffuse
  vec3 normal = normalize(Normal);
  vec3 lightDirection = normalize(-object_light.light.direction);
  float diffuseDistance = max(dot(normal,lightDirection),0.0);
  vec3 diffuse = object_light.light.diffuse * (diffuseDistance * object.material.diffuse);

  //Specular
  vec3 viewDirection = normalize(object.viewPos - FragPos);
  vec3 reflectDirection = reflect(-object_light.light.position,normal);
  float spec = pow(max(dot(viewDirection,reflectDirection),0.0), object.material.shininess);
  vec3 specular = object_light.light.specular * (spec * object.material.specular);

  vec3 result = ambient + diffuse + specular;*/

  //Point Light
  /*vec3 ambient = object_light.light.ambient * texture(normalMap,FragUV).rgb;

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

  float distance = length(object_light.light.position - FragPos);
  float attenuation = 1.0 / (object_light.light.constant + object_light.light.linear * distance + object_light.light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  vec3 result = ambient + diffuse + specular;

  outColor = vec4(result,1.0);*/

  //Spot Light
  /*vec3 ambient = vec3(0.1,0.1,0.1) * texture(normalMap,FragUV).rgb;

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

  float theta = dot(lightDirection, normalize(object_light.light.direction));
  float epsilon = object_light.light.cut_off - object_light.light.outer_cut_off;
  float intensity = clamp((theta - object_light.light.outer_cut_off) / epsilon,0.0,1.0);
  diffuse *= intensity;
  specular *= intensity;

  float distance = length(object_light.light.position - FragPos);
  float attenuation = 1.0 / (object_light.light.constant + object_light.light.linear * distance + object_light.light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  vec3 result = ambient + diffuse + specular;*/

  outColor = vec4(result,1.0);
}
