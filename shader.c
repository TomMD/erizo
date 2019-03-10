#include "shader.h"

GLuint shader_build(const GLchar* src, GLenum type) {
    const GLuint shader = glCreateShader(type);
    glShaderSource(shader, 1, &src, NULL);
    glCompileShader(shader);

    GLint status;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &status);
    if (status != GL_TRUE) {
        GLint len = 0;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &len);

        GLchar* buf = malloc(len + 1);
        glGetShaderInfoLog(shader, len, NULL, buf);
        fprintf(stderr, "Failed to build shader: %s\n", buf);
        free(buf);
    }

    return shader;
}

GLuint shader_link(GLuint vs, GLuint gs, GLuint fs) {
    GLuint program = glCreateProgram();

    glAttachShader(program, vs);
    glAttachShader(program, gs);
    glAttachShader(program, fs);

    glLinkProgram(program);

    GLint status;
    glGetProgramiv(program, GL_LINK_STATUS, &status);
    if (status != GL_TRUE) {
        GLint len = 0;
        glGetProgramiv(program, GL_INFO_LOG_LENGTH, &len);

        GLchar* buf = malloc(len + 1);
        glGetProgramInfoLog(program, len, NULL, buf);
        fprintf(stderr, "Failed to link program: %s\n", buf);
        free(buf);
    }

    return program;
}