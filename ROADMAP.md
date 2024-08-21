# Features

## PSD
- Parse psd to get each layer as a png
  - Trim all png files and record their offset to keep them in place

## Bones
We are going to need custom relationships outside of parent/child because of the limitations with transform propagation. It might be best to approach this with a constraint system, where different constraints can be applied to a bone to follow different aspects of another bone. The default can be to constrain the Translation, Rotation, and Scale to the "Parent". This should work well with the ECS system.  

- Custom component-based constraint system

## UI
I'm not loving SickleUI, because I have too many opinions about little UX interactions. This is also a chance to evaluate what the best presentation would be for the bone hierarchy. If it's not really a tree, but instead a graph of relationships, a list view might not be sufficient anyway. It might be best to make some kind of node graph system to be able to visualize the relationships.

In Blender and that course, the guy differentiated between bones that distorted the mesh, bones that were complex mechanisms with constraints, and UI bones for the actual control of the rig. I wonder if these all are really necessary if we focus on a good system of constraints.

