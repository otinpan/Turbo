#version 450

const int MAX_POINT_LIGHTS = 8;
const int MAX_SPOT_LIGHTS = 8;

layout(set = 1, binding = 0) uniform sampler2D texSampler;

layout(set = 2, binding = 0) uniform LightUniform {
  vec4 directionalDirection;
  vec4 directionalColor;
  vec4 ambientColor;

  // x = spot light count, y = point light count
  vec4 lightParams;

  vec4 pointLightPositions[MAX_POINT_LIGHTS];
  vec4 pointLightColors[MAX_POINT_LIGHTS];

  vec4 spotLightPositions[MAX_SPOT_LIGHTS];
  vec4 spotLightDirections[MAX_SPOT_LIGHTS];
  vec4 spotLightColors[MAX_SPOT_LIGHTS];
  vec4 spotLightConeParams[MAX_SPOT_LIGHTS];
} light;

layout(push_constant) uniform PushConstants {
  layout(offset = 64) vec4 materialColor;
  vec4 materialFlags;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;
layout(location = 2) in vec3 fragNormal;
layout(location = 3) in vec3 fragWorldPos;

layout(location = 0) out vec4 outColor;

void main() {
  vec4 base = pcs.materialColor;

  if (pcs.materialFlags.x > 0.5) {
    base *= texture(texSampler, fragTexCoord);
  }

  vec3 normal = normalize(fragNormal);
  vec3 lighting = light.ambientColor.rgb * light.ambientColor.a;

  vec3 dirLightDir = normalize(-light.directionalDirection.xyz);
  float dirDiffuse = max(dot(normal, dirLightDir), 0.0);
  lighting += light.directionalColor.rgb * light.directionalColor.a * dirDiffuse;

  int pointLightCount = int(light.lightParams.y);

  for (int i = 0; i < pointLightCount; i++) {
    vec3 toLight = light.pointLightPositions[i].xyz - fragWorldPos;
    float distance = length(toLight);
    vec3 pointLightDir = normalize(toLight);

    float radius = light.pointLightPositions[i].w;
    float attenuation = clamp(1.0 - distance / radius, 0.0, 1.0);
    attenuation *= attenuation;

    float pointDiffuse = max(dot(normal, pointLightDir), 0.0);

    lighting += light.pointLightColors[i].rgb
      * light.pointLightColors[i].a
      * pointDiffuse
      * attenuation;
  }

  int spotLightCount = int(light.lightParams.x);

  for (int i = 0; i < spotLightCount; i++) {
    vec3 toLight = light.spotLightPositions[i].xyz - fragWorldPos;
    float distance = length(toLight);
    vec3 lightDir = normalize(toLight);

    float radius = light.spotLightPositions[i].w;
    float attenuation = clamp(1.0 - distance / radius, 0.0, 1.0);
    attenuation *= attenuation;

    vec3 spotForward = normalize(light.spotLightDirections[i].xyz);
    float theta = dot(normalize(-lightDir), spotForward);

    float innerCutoff = light.spotLightConeParams[i].x;
    float outerCutoff = light.spotLightConeParams[i].y;
    float cone = smoothstep(outerCutoff, innerCutoff, theta);

    float diffuse = max(dot(normal, lightDir), 0.0);

    lighting += light.spotLightColors[i].rgb
      * light.spotLightColors[i].a
      * diffuse
      * attenuation
      * cone;
  }

  vec3 rgb = base.rgb * fragColor * lighting;

  outColor = vec4(rgb, base.a);
}
