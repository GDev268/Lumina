@echo off
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/default/default_shader.vert -o shaders/default/default_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/default/default_shader.frag -o shaders/default/default_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/shadow/shadow_map_shader.vert -o shaders/shadow/shadow_map_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/shadow/shadow_map_shader.frag -o shaders/shadow/shadow_map_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/skybox/skybox_shader.vert -o shaders/skybox/skybox_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/skybox/skybox_shader.frag -o shaders/skybox/skybox_shader.frag.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/gui/gui_shader.vert -o shaders/gui/gui_shader.vert.spv
"C:\VulkanSDK\1.3.268.0\Bin\glslc.exe"  shaders/gui/gui_shader.frag -o shaders/gui/gui_shader.frag.spv