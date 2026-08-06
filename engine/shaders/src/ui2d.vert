#version 450

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec2 fragTexCoord;

layout(push_constant) uniform PushConstants {
    vec4 transform;
} pcs;

void main() {
    vec2 pos = inPosition * pcs.transform.zw + pcs.transform.xy;
    gl_Position = vec4(pos.x, -pos.y, 0.0, 1.0);
    fragColor = vec4(inColor, 1.0);
    fragTexCoord = inTexCoord;
}
