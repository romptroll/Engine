#shader vertex
#version 330 core

layout(location = 0) in vec3 v_pos;
layout(location = 1) in vec3 v_size;
layout(location = 2) in int v_color;
layout(location = 3) in vec4 v_mat_0;
layout(location = 4) in vec4 v_mat_1;
layout(location = 5) in vec4 v_mat_2;
layout(location = 6) in vec4 v_mat_3;


out mat4 mat;
out vec3 size;
out int tight_color;

void main() {
    gl_Position = vec4(v_pos, 1.0);
    size = v_size;
    tight_color = v_color;
    mat[0] = v_mat_0;
    mat[1] = v_mat_1;
    mat[2] = v_mat_2;
    mat[3] = v_mat_3;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 64) out;

in int tight_color[];
in mat4 mat[];
in vec3 size[];

out vec4 color;

const int DRAW_PLANE = 0;
const int DRAW_LINE = 1;
const int DRAW_SPHERE = 2;
const int DRAW_CUBE = 3;

uniform int u_primitive = 0;
uniform int u_sphere_detail = 100;
uniform float u_line_width = 0.01;


vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void draw_plane(vec3 pos1, vec3 pos2, vec3 pos3) {
    vec3 pos4 = pos2 - (pos2 - pos1) - (pos2 - pos3);

    gl_Position = vec4(pos1, 1.0) * mat[0];
    EmitVertex();
    
    gl_Position = vec4(pos2, 1.0) * mat[0];
    EmitVertex();

    gl_Position = vec4(pos4, 1.0) * mat[0];
    EmitVertex();

    gl_Position = vec4(pos3, 1.0) * mat[0];
    EmitVertex();

    color.g += 0.1;
}

void draw_cube(float x, float y, float z, float width, float height, float depth) {
    vec3 pos1 = vec3(x, y, z);
    vec3 pos2 = vec3(x, y + height, z);
    vec3 pos3 = vec3(x + width, y + height, z);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // FRONT

    pos1 = vec3(x, y, z + depth);
    pos2 = vec3(x, y + height, z + depth);
    pos3 = vec3(x + width, y + height, z + depth);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // BAK

    pos1 = vec3(x, y, z);
    pos2 = vec3(x, y + height, z);
    pos3 = vec3(x, y + height, z + depth);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // LEFT

    pos1 = vec3(x + width, y, z);
    pos2 = vec3(x + width, y + height, z);
    pos3 = vec3(x + width, y + height, z + depth);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // RIGHT

    pos1 = vec3(x, y + height, z);
    pos2 = vec3(x, y + height, z + depth);
    pos3 = vec3(x + width, y + height, z + depth);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // TOP

    pos1 = vec3(x, y, z);
    pos2 = vec3(x, y, z + depth);
    pos3 = vec3(x + width, y, z + depth);
    draw_plane(pos1, pos2, pos3); EndPrimitive(); // BOTTOM
}

void draw_sphere(float x, float y, float z, float width, float height, float depth) {
    for(int i = 0; i < u_sphere_detail+1; i++) {
        float nx = cos(float(i) / float(u_sphere_detail) * 2.0 * 3.14) * width / 2.0;
        float ny = sin(float(i) / float(u_sphere_detail) * 2.0 * 3.14) * height / 2.0;
        nx = x + width / 2.0 + nx;
        ny = y + height / 2.0 + ny;

        gl_Position = vec4(vec3(nx, ny, 1.0), 1.0) * mat[0];
        EmitVertex();

        gl_Position = vec4(vec3(x + width / 2.0, y + height / 2.0, 1.0), 1.0) * mat[0];
        EmitVertex();
    }
    float nx = cos(float(0) / float(u_sphere_detail) * 2.0 * 3.14) * width / 2.0;
    float ny = sin(float(0) / float(u_sphere_detail) * 2.0 * 3.14) * height / 2.0;
    nx = x + width / 2.0 + nx;
    ny = y + height / 2.0 + ny;

    gl_Position = vec4(vec3(nx, ny, 1.0), 1.0) * mat[0];
    EmitVertex();

    gl_Position = vec4(vec3(x + width / 2.0, y + height / 2.0, 1.0), 1.0) * mat[0];
    EmitVertex();
}

void draw_line(float x1, float y1, float z1, float x2, float y2, float z2) {
    float x_len = x1 - x2;
    float y_len = y1 - y2;
    
    float a = atan(y_len / x_len);
    float pi = 3.14 / 2.0;

    vec2 pos1 = vec2(x1 + cos(a + pi) * u_line_width, y1 + sin(a + pi) * u_line_width); //(0, 0)
    vec2 pos2 = vec2(x1 + cos(a - pi) * u_line_width, y1 + sin(a - pi) * u_line_width); //(1, 0)
    vec2 pos3 = vec2(x2 + cos(a + pi) * u_line_width, y2 + sin(a + pi) * u_line_width); //(0, 1)
    vec2 pos4 = vec2(x2 + cos(a - pi) * u_line_width, y2 + sin(a - pi) * u_line_width); //(1, 1)

    ///////////////////////////////////////////////////////

    //(0, 0)
    gl_Position = vec4(vec3(pos1.x, pos1.y, 1.0), 1.0) * mat[0];
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(vec3(pos2.x, pos2.y, 1.0), 1.0) * mat[0];
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(vec3(pos3.x, pos3.y, 1.0), 1.0) * mat[0];
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(vec3(pos4.x, pos4.y, 1.0), 1.0) * mat[0];
    EmitVertex();
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;
    vec3 siz = size[0];

    color = col;

    ///////////////////////////////////////////////////////
    if(u_primitive == DRAW_SPHERE) {
        draw_sphere(pos.x, pos.y, pos.z, siz.x, siz.y, siz.z);
    }
    else if(u_primitive == DRAW_LINE) {
        draw_line(pos.x, pos.y, pos.z, siz.x, siz.y, siz.z);
    }
    else if(u_primitive == DRAW_CUBE) {
        draw_cube(pos.x, pos.y, pos.z, siz.x, siz.y, siz.z);
    }
    else if(u_primitive == DRAW_PLANE) {
        vec3 pos1 = pos.xyz;
        vec3 pos2 = vec3(pos.x, pos.y + siz.y, pos.z);
        vec3 pos3 = vec3(pos.x + siz.x, pos.y + siz.y, pos.x);
        
        draw_plane(pos1, pos2, pos3);
    }
   
}

#shader fragment
#version 330 core

in vec4 color;

layout(location = 0) out vec4 out_color;

void main() {
	out_color = color;
}