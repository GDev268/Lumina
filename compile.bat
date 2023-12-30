@echo off
::"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe" shaders/default/default_shader.vert -o shaders/default/default_shader.vert.spv
::"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/default/default_shader.frag -o shaders/default/default_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe" shaders/canvas/canvas_shader.vert -o shaders/canvas/canvas_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/canvas/canvas_shader.frag -o shaders/canvas/canvas_shader.frag.spv
