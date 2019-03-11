#include "log.h"
#include "model.h"
#include "shader.h"

const GLchar* MODEL_VS_SRC = GLSL(330,
layout(location=0) in vec3 pos;

void main() {
    gl_Position = vec4(pos, 1.0f);
}
);

const GLchar* MODEL_GS_SRC = GLSL(330,
layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

uniform mat4 proj;
uniform mat4 model;

out vec3 vert_norm;
out vec3 pos_bary;

void main() {
    vec3 a = gl_in[0].gl_Position.xyz;
    vec3 b = gl_in[1].gl_Position.xyz;
    vec3 c = gl_in[2].gl_Position.xyz;

    vec3 na = cross(a - b, c - b);
    vec3 nb = cross(b - c, a - c);
    vec3 nc = cross(c - a, b - a);

    vert_norm = normalize(na + nb + nc);

    pos_bary = vec3(1.0f, 0.0f, 0.0f);
    gl_Position = proj * model * gl_in[0].gl_Position;
    EmitVertex();

    pos_bary = vec3(0.0f, 1.0f, 0.0f);
    gl_Position = proj * model * gl_in[1].gl_Position;
    EmitVertex();

    pos_bary = vec3(0.0f, 0.0f, 1.0f);
    gl_Position = proj * model * gl_in[2].gl_Position;
    EmitVertex();
}
);

const GLchar* MODEL_FS_SRC = GLSL(330,
in vec3 vert_norm;
in vec3 pos_bary;

out vec4 out_color;

void main() {
    out_color = vec4(abs(vert_norm), 1.0f) * 0.8f
              + vec4(pos_bary, 1.0f) * 0.2f;
}
);

void model_init(model_t* model) {
    model->num_triangles = 0;
    model->vbo = 0;
    glGenVertexArrays(1, &model->vao);

    GLuint vs = shader_build(MODEL_VS_SRC, GL_VERTEX_SHADER);
    GLuint gs = shader_build(MODEL_GS_SRC, GL_GEOMETRY_SHADER);
    GLuint fs = shader_build(MODEL_FS_SRC, GL_FRAGMENT_SHADER);
    model->prog = shader_link(vs, gs, fs);
    model->u_proj = glGetUniformLocation(model->prog, "proj");
    model->u_model = glGetUniformLocation(model->prog, "model");

    log_trace("Initialized model");
}

void model_draw(model_t* model, const float* proj) {
    glEnable(GL_DEPTH_TEST);
    glUseProgram(model->prog);
    glBindVertexArray(model->vao);
    glUniformMatrix4fv(model->u_proj, 1, GL_FALSE, proj);
    glUniformMatrix4fv(model->u_model, 1, GL_FALSE, model->mat);
    glDrawArrays(GL_TRIANGLES, 0, model->num_triangles * 3);
}
