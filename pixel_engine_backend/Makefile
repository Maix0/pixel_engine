# This Makefile generates SPIR-V shaders from GLSL shaders in the examples.
# Copied from the wgpu-rs Makefile for examples' shader compilation
# Adapted for compiling shader in `shader` folder
shader_compiler = glslangValidator

# All input shaders.
glsls = $(wildcard shaders/*.vert shaders/*.frag shaders/*.comp)

# All SPIR-V targets.
spirvs = $(addsuffix .spv,$(glsls))

.PHONY: default
default: $(spirvs)

# Rule for making a SPIR-V target.
$(spirvs): %.spv: %
	$(shader_compiler) -V $< -o $@

.PHONY: clean
clean:
	rm -f $(spirvs)
