#version 450

layout(location = 0) in vec3 inPosition;

layout(set = 0, binding = 0) uniform GlobalUniformObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) out vec3 fragTexCoord;

void main() {
    fragTexCoord = inPosition;

    mat4 viewNoTranslation = mat4(mat3(ubo.view));
    vec4 pos = ubo.proj * viewNoTranslation * vec4(inPosition, 1.0);

    gl_Position = pos.xyww;
}