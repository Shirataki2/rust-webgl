
layout (location = 0) in vec3 iPosition;
layout (location = 1) in vec3 iColor;

out vec3 oColor;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

void main() {
    gl_Position = uProjection * uView * uModel * vec4(iPosition, 1.0f);
    oColor = iColor;
}
