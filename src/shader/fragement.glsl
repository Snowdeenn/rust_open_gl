#version 330 core

in vec3 vColor;
in vec2 vUV;
out vec4 fragColor;

uniform sampler2D  uTexture;
uniform sampler2D uTexture2;

void main(){
    vec4 colTex1 = texture(uTexture, vUV);
    vec4 colTex2 = texture(uTexture2, vUV);
    fragColor = mix(colTex1, colTex2, 0.5);
}