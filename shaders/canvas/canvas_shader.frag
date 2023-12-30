#version 450

layout(location = 0) in vec2 fragTexCoord;

layout(binding = 0) uniform sampler2D ImageTexture;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = texture(ImageTexture, fragTexCoord);
}