#shader vertex
#version 330 core

layout(location = 0) in vec3 pos;
out vec4 color;

uniform mat4 u_MVP;
//uniform mat4 u_Model;

void main() {

	gl_Position = vec4(pos, 1.0) * u_MVP;

	//color = vec4(((pos + 1.0) / 4.0) + 0.25, 1.0);
	color = vec4(pos.x, pos.y, 1.0, 1.0);
};

#shader fragment
#version 330 core
out vec4 FragColor;
in vec4 color;

uniform vec4 u_color;

void main() {
	FragColor = u_color;
};
