#version 300 es

// fragment shaders don't have a default precision so we need
// to pick one. mediump is a good default. It means "medium precision"
precision mediump float;

// our texture
uniform sampler2D u_image;

// the texCoords passed in from the vertex shader.
in vec2 v_texCoord;

// we need to declare an output for the fragment shader
out vec4 outColor;

void main() {
  vec2 onePixel = vec2(1) / vec2(textureSize(u_image, 0));

  // average the left, middle, and right pixels.
  outColor = (
      texture(u_image, v_texCoord) +
      texture(u_image, v_texCoord + vec2( onePixel.x, 0.0)) +
      texture(u_image, v_texCoord + vec2(-onePixel.x, 0.0))) / 3.0;
}