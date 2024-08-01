#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

// out
varying vec2 uv;
varying lowp vec4 color;
varying lowp float dist;

uniform mat4 Model;
uniform mat4 Projection;
uniform lowp vec2 player_gl_pos;

uniform lowp vec3 player_world_coord;
uniform lowp vec3 block_world_coord;

void main() {
    color = color0 / 255.0;
    lowp vec4 pos = Projection * Model * vec4(position, 1);
    gl_Position = pos;
    uv = texcoord;
}
