#version 150 core

in vec3 a_Pos;
out vec4 v_Color;

uniform Locals {
	mat4 u_Model;
	mat4 u_View;
	mat4 u_Proj;
};

void main() {
    gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 1.0);
    v_Color = vec4(1.0, 1.0, 1.0, 1.0);
}