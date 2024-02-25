#version 450

layout(location = 0) in vec3 fragUV;
layout(location = 0) out vec4 outColor;

// cubemap-color
layout(set = 0, binding = 1) uniform samplerCube skybox;

void main() {    
    outColor = texture(skybox, fragUV);
}
