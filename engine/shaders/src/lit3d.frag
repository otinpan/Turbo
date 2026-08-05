#version 450

layout(set = 1, binding = 0) uniform sampler2D texSampler;

layout(set = 2, binding = 0) uniform LightUniform {
  vec4 lightDirection;
  vec4 lightColor;
  vec4 ambientColor;
} light;

layout(push_constant) uniform PushConstants {
  layout(offset = 64) vec4 materialColor;
  vec4 materialFlags;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;
layout(location = 2) in vec3 fragNormal;

layout(location = 0) out vec4 outColor;

void main() {
  vec4 base = pcs.materialColor;

  if (pcs.materialFlags.x > 0.5) {
    base *= texture(texSampler, fragTexCoord);
  }

  vec3 normal = normalize(fragNormal);

  vec3 lightDir = normalize(-light.lightDirection.xyz);

  float diffuse = max(dot(normal, lightDir), 0.0);

  vec3 lighting = light.ambientColor.rgb * light.ambientColor.a
    + light.lightColor.rgb * light.lightColor.a * diffuse;
  vec3 rgb = base.rgb * fragColor * lighting;

  outColor = vec4(rgb, base.a);
}
