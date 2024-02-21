@echo off
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe" shaders/default_shader.vert -o shaders/default_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/default_shader.frag -o shaders/default_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/light_cube_shader.vert -o shaders/light_cube_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/light_cube_shader.frag -o shaders/light_cube_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/shadow_map_shader.vert -o shaders/shadow_map_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/shadow_map_shader.frag -o shaders/shadow_map_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/skybox_shader.vert -o shaders/skybox_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/skybox_shader.frag -o shaders/skybox_shader.frag.spv
