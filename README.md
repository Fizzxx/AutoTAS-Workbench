# AutoTAS-Workbench

A lightweight platform for applying reinforcement learning to video games.

This is a work in progress, and is still in the very early stages of development.
Many features have not yet been implemented.

## Features

### Capture image data for each frame
You will be able to connect to any ongoing process and collect image data frame-by-frame.
To accomplish this, two methods will be available:
1. Hooking to the renderer via dll injection: 
   This will collect the image data for each frame prior to rendering,
   allowing for ultra-fast delivery to your training loop.

   NOTE: dll injection may go against the terms of service for some applications, and will be flagged by any 
    anti-cheat software. Prior to using this feature, confirm that you are allowed by the terms of service
    to modify game files.
   
2. Basic screen capture: when simplicity is necessary, or preferred over speed (slower; less risky)
 
### Execute your Python ML code
Create your own preprocessing scripts, training loops, and more using your favorite libraries, 
and AutoTAS-Workbench will be able to seamlessly integrate them within the pipeline. 
Simply point AutoTAS-Workbench to your .py files, and specify the desired order in which the data will flow.

### Design your own metrics and monitor them alongside training
Visualize any metrics you wish within the GUI. Specify the source and see them plotted 
as each frame is processed during training or evaluation.

### Simulate realistic input
AutoTAS-Workbench will be able to provide input to your application as your model makes predictions.
This could be either keyboard, mouse, or controller input. 
You will also have the ability to add any desired additional latency in milliseconds, 
should you wish to more accurately represent reaction times.


## Progress
- Completed (most recent listed first)
  - DLL injection test
  - Selection of target process
  - Basic GUI elements in place

- Upcoming
  - Hooking the (DirectX) renderer
  - Retrieving frame data
  - Formatting frame data to commonly used tensor formats
