#shader vertex
#version 330 core

layout(location = 0) in vec4 v_bounds;
layout(location = 1) in vec4 v_uv_bounds;
layout(location = 2) in int v_color;
layout(location = 3) in vec3 v_mat_0;
layout(location = 4) in vec3 v_mat_1;
layout(location = 5) in vec3 v_mat_2;

out mat3 mat;
out vec4 uv_bounds;
out int tight_color;

void main() {
    gl_Position = v_bounds;
    uv_bounds = v_uv_bounds;
    tight_color = v_color;
    mat[0] = v_mat_0;
    mat[1] = v_mat_1;
    mat[2] = v_mat_2;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 6) out;

in vec4 uv_bounds[];
in int tight_color[];
in mat3 mat[];

out vec2 uv;
out vec4 color;

vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;

    color = col;

    ///////////////////////////////////////////////////////

    //pos.x *= 0.1;
    //pos.y *= 0.1;
    //pos.z
    

    //(0, 0)
    gl_Position = vec4(vec3(pos.x, pos.y, 1.0) * mat[0], 1.0);
    uv = vec2(uv_bounds[0].x, uv_bounds[0].y);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(vec3(pos.x + pos.z, pos.y, 1.0) * mat[0], 1.0);
    uv = vec2(uv_bounds[0].x + uv_bounds[0].z, uv_bounds[0].y);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(vec3(pos.x, pos.y + pos.w, 1.0) * mat[0], 1.0);
    uv = vec2(uv_bounds[0].x, uv_bounds[0].y + uv_bounds[0].w);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(vec3(pos.x + pos.z, pos.y + pos.w, 1.0) * mat[0], 1.0);
    uv = vec2(uv_bounds[0].x + uv_bounds[0].z, uv_bounds[0].y + uv_bounds[0].w);
    EmitVertex();
}

#shader fragment
#version 330 core

in vec2 uv;
in vec4 color;

layout(location = 0) out vec4 out_color;

uniform sampler2D u_Texture;

void main() {
    vec4 texColor = texture(u_Texture, uv);
	out_color = texColor * color;
}