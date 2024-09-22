#version 100
#extension GL_OES_sample_variables : enable
precision lowp float;

varying vec2 uv;
varying lowp vec4 color;
varying lowp float dist;

uniform sampler2D Texture;
uniform vec2 resolution; // screen size in pixels
uniform vec2 resolution_cam; // todo: remove screen size in camera world coordiantes
uniform vec2 mouse; // mouse position on the screen
uniform vec2 camera_zoom; // camera_zoom for consistent screen size normalization
uniform lowp vec2 player_gl_pos; // player position on screen (its located on the base of the player not center)
uniform lowp float player_dist; // todo: remove
uniform lowp vec3 player_world_pos; // player position in the world (x,y,z)
uniform lowp vec3 block_world_pos; // position in the world (x,y,z) of the entire block being rendered
uniform int player_hidble;
uniform int block_behind_player; // whether if block is behind the player

// compares two tiles to determain which one appears on top
// if positive $lhs is on top of $rhs
// if negetive $rhs is on top of $lhs
// if 0 they are the same cordinates
int cmp_tile(vec3 lhs_in, vec3 rhs_in) {
    vec3 lhs = vec3(floor(lhs_in.x), floor(lhs_in.y), floor(lhs_in.z));
    vec3 rhs = vec3(floor(rhs_in.x), floor(rhs_in.y), floor(rhs_in.z));
    float r = (lhs.x + lhs.y + lhs.z) - (rhs.x + rhs.y + rhs.z);
    if (r < 0.0) return -1;
    if (r > 0.0) return +1;
    return 0;
}

// https://computergraphics.stackexchange.com/questions/5724/glsl-can-someone-explain-why-gl-fragcoord-xy-screensize-is-performed-and-for
float player_glass() {
    if (block_behind_player == 1 || player_hidble == 1) {
       return 1.0;
    }
    lowp float a = 1.0;
    lowp float r = 40.0;
    lowp float d = 0.0;
    // camera normalizer (should be close to the default camera size times a 10^n val like 0.00006 -> 600,
    // this value is eyeballed)
    lowp float c = camera_zoom.y * 600.0;
    lowp float i = 8.0; // blend multiplier higher less blend
    r = r * c;
    d = distance(player_gl_pos + /*offset so player spirtes at center*/ vec2(0.0, 18.0 /*close to size of players sprite*/ * c), gl_FragCoord.xy);
    bool b = true;
    //TODO: make this so it only happens when the player is for sure obstructed by a block
    // if (d > r / 2.0) {
    // b = cmp_tile(block_world_pos - vec3(uv.x, -uv.y, 0.0), player_world_pos) > 0;
    // }
    if (
        // only cut if the block is being render over the player
        block_world_pos.z >= player_world_pos.z &&
            // cmp_tile(block_world_pos, player_world_pos) > 0 &&
            b &&
            d < r
    ) {
        a = pow((d / r), i);
    }
    return a;
}

void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, player_glass()) * color * texture2D(Texture, uv);
}
