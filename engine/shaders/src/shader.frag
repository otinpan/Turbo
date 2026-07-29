#version 450
// binding from uniform.rs
layout(binding = 1) uniform sampler2D texSampler;

layout(push_constant) uniform PushConstants {
  layout(offset = 64) vec4 materialColor;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
  vec4 texColor = texture(texSampler, fragTexCoord);
  vec3 baseColor = texColor.rgb * fragColor * pcs.materialColor.rgb;
  outColor = vec4(baseColor, texColor.a * pcs.materialColor.a);
}