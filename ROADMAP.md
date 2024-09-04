# Features

## OpenRaster
- Parse .ora to get each layer as a png
  - Trim all png files and record their offset to keep them in place

## Bones
We are going to need custom relationships outside of parent/child because of the limitations with transform propagation. It might be best to approach this with a constraint system, where different constraints can be applied to a bone to follow different aspects of another bone. The default can be to constrain the Translation, Rotation, and Scale to the "Parent". This should work well with the ECS system.  

- Custom component-based constraint system

## UI
I'm not loving SickleUI, because I have too many opinions about little UX interactions. This is also a chance to evaluate what the best presentation would be for the bone hierarchy. If it's not really a tree, but instead a graph of relationships, a list view might not be sufficient anyway. It might be best to make some kind of node graph system to be able to visualize the relationships.

In Blender and that course, the guy differentiated between bones that distorted the mesh, bones that were complex mechanisms with constraints, and UI bones for the actual control of the rig. I wonder if these all are really necessary if we focus on a good system of constraints.

## Tessellation
I am still really interested in being able to advance on Asesprite's drawing across instanced tiles, but with a better brush engine that works for high-res drawing and painting.

- Figure out basic drawing engine.
- Multiple mesh instance drawing.

## GPU for Drawing
Look into using compute shaders for drawing to the texture.
https://github.com/bevyengine/bevy/blob/release-0.14.1/examples/shader/compute_shader_game_of_life.rs

We have a fast drawing shader! Some thoughts on next steps:
- Add some camera controls - pan, zoom
- Add sprite movement controls to change the transform
- Figure out drawing on multiple sprites, and multiple instances of the same sprite
- Add different brush modes, sizes, and color picking
