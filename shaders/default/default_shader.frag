#version 450
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
  vec3 color;
  vec3 rotation;  

  float intensity;
  float spot_size;
  
  float linear;
  float quadratic;

  uint type;
};

//64
layout(set = 0, binding = 1) uniform MaterialInfo {
  Material material;
  vec3 viewPos;
} object;

//64
layout(set = 0, binding = 2) uniform LightInfo {
  Light light[999];
} object_light;

//color
layout(set = 0, binding = 3) uniform sampler2D colorMap;

//color
layout(set = 0, binding = 4) uniform sampler2D normalMap;

//color
layout(set = 0, binding = 5) uniform sampler2D specularMap;


//64
layout(set = 0, binding = 7) uniform LightSpaceMatrices {
  mat4 matrix[999];
} light_mat;

vec3 CalculateDirectionalLight(Light light, vec3 normal,vec3 fragPos, vec3 viewDir,float shadow);
vec3 CalculatePointLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalculateSpotLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir);

void main() {
  vec3 result = vec3(0.0, 0.0, 0.0);

  vec3 normal = normalize(Normal);
  vec3 viewDirection = normalize(object.viewPos - FragPos);

  for (int i = 0; i < 2; i++) {
    if (object_light.light[i].type == 0) {
      result += CalculateDirectionalLight(object_light.light[i], normal, FragPos, viewDirection,1.0);
    } else if (object_light.light[i].type == 1) {
      result += CalculatePointLight(object_light.light[i], normal, FragPos, viewDirection);
    } else if (object_light.light[i].type == 2) {
      result += CalculateSpotLight(object_light.light[i], normal, FragPos, viewDirection);
    }
  }

  outColor = vec4(result, 1.0);
}

vec3 CalculateDirectionalLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDirection, float shadow) {
  vec3 ambient = light.color * (texture(colorMap, FragUV).rgb + object.material.ambient);

  vec3 lightDirection = normalize(-light.rotation);
  float diffuseDistance = max(dot(normal, lightDirection), 0.0);
  
  float metallic = object.material.shininess;
  vec3 diffuse = light.color * mix(vec3(0.05), (texture(colorMap, FragUV).rgb + object.material.diffuse), metallic);

  vec3 reflectDirection = reflect(-lightDirection, normal);
  vec3 specular = light.color * (metallic * (texture(specularMap, FragUV).rgb + object.material.specular));

  ambient *= light.intensity;
  diffuse *= light.intensity;
  specular *= light.intensity;

  return (ambient + (1.0 - shadow) * (diffuse + specular));
}

vec3 CalculatePointLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDirection) {
  vec3 ambient = light.color * (texture(colorMap, FragUV).rgb + object.material.ambient);

  vec3 lightDirection = normalize(light.position - fragPos);
  float diffuseDistance = max(dot(normal, lightDirection), 0.0);
  
  float metallic = object.material.shininess;
  vec3 diffuse = light.color * mix(vec3(0.05), (texture(colorMap, FragUV).rgb + object.material.diffuse), metallic);

  vec3 reflectDirection = reflect(-lightDirection, normal);
  vec3 specular = light.color * (metallic * (texture(specularMap, FragUV).rgb + object.material.specular));

  float distance = length(light.position - fragPos);
  float attenuation = light.intensity / (1.0 + light.linear * distance + light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}

vec3 CalculateSpotLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDirection) {
  vec3 ambient = light.color * (texture(colorMap, FragUV).rgb + object.material.ambient);

  vec3 lightDirection = normalize(light.position - fragPos);
  float diffuseDistance = max(dot(normal, lightDirection), 0.0);
  
  float metallic = object.material.shininess;
  vec3 diffuse = light.color * mix(vec3(0.05), (texture(colorMap, FragUV).rgb + object.material.diffuse), metallic);

  vec3 reflectDirection = reflect(-lightDirection, normal);
  vec3 specular = light.color * (metallic * (texture(specularMap, FragUV).rgb + object.material.specular));

  float theta = dot(lightDirection, normalize(-light.rotation));
  float cutOff = cos(radians(light.spot_size));
  float outerCutOff = cos(radians(light.spot_size + 15));
  float epsilon = cutOff - outerCutOff;
  float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

  ambient *= intensity;
  diffuse *= intensity;
  specular *= intensity;

  float distance = length(light.position - fragPos);
  float attenuation = light.intensity / (1.0 + light.linear * distance + light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}
