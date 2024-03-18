in vec4 col;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

vec4 sticker(vec4 colors, vec2 uv) {
    if (uv.x < 0.5) {
        return colors;
    }
    return vec4(0.0, 0.0, 0.0, 1.0);
}

void main() {
    // outColor = sticker(col, uvs);
    outColor = col;
}
