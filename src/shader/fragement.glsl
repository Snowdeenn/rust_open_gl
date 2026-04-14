#version 330 core

in vec3 vColor;
in vec2 vUV;
in vec3 vNormal;
in vec3 vFragPos;
out vec4 fragColor;

uniform sampler2D  uTexture;
uniform sampler2D uTexture2;
uniform vec3 uLightColor;
uniform vec3 uLightPos;
uniform vec3 uCamPos;

void main(){
    vec4 colTex1 = texture(uTexture, vUV);
    vec4 colTex2 = texture(uTexture2, vUV);
    vec3 lightDirection = normalize(uLightPos - vFragPos);
    vec3 diffus = max(dot(vNormal, lightDirection), 0.0) * uLightColor;

    vec3 viewDirection  = normalize(uCamPos - vFragPos);
    vec3 reflectDir     = reflect(-lightDirection, vNormal);
    float spec           = pow(max(dot(viewDirection, reflectDir), 0.0), 8.0);
    vec3 speculaire     = spec * uLightColor;

    vec3 ambient = 0.6 * uLightColor;
    vec4 colTex = mix(colTex1, colTex2, 0.5);
    fragColor = vec4(ambient + diffus + speculaire, 1.0) * colTex;
}