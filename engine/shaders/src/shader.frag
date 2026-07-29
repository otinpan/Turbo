#version 450
// binding from uniform.rs
layout(binding = 1) uniform sampler2D texSampler;

layout(push_constant) uniform PushConstants {
  layout(offset = 64) vec4 materialColor;
  vec4 materialFlags;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
  vec4 base=pcs.materialColor;
  if (pcs.materialFlags.x>0.5){
    base*=texture(texSampler,fragTexCoord);
  }
  outColor = vec4(base.rgb*fragColor,base.a);
}