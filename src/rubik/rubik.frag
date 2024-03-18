in vec4 col;
in vec3 nor;
in vec2 uvs;

uniform vec2 stickerData;

layout (location = 0) out vec4 outColor;

vec4 sticker(vec4 colors, vec2 uv) {
    vec2 shifted = uv*2.0 - vec2(1, 1);
    if ((abs(shifted.x) < stickerData.x && abs(shifted.y) < stickerData.x + stickerData.y) ||
        (abs(shifted.x) < stickerData.x + stickerData.y && abs(shifted.y) < stickerData.x) ||
        dot(abs(shifted) - vec2(stickerData.x, stickerData.x), abs(shifted) - vec2(stickerData.x, stickerData.x)) < stickerData.y * stickerData.y) {
        return colors;
    }
    return vec4(0.0, 0.0, 0.0, 1.0);
}

void main() {
    outColor = sticker(col, uvs);
    // outColor = col;
    outColor.rgb = color_mapping(outColor.rgb);
}
