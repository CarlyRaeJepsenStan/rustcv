# rustcv
Edge detection and other computer vision thingies in rust. 

## 1: Edge Detection
- Load image with image-rs
- Resize to square dimensions
- Apply Laplace or Sobel kernels 
  - Use inner or outer product to get elementwise product sum of kernel and image

References: 
https://towardsdatascience.com/convolution-neural-networks-a-beginners-guide-implementing-a-mnist-hand-written-digit-8aa60330d022

https://inst.eecs.berkeley.edu/~cs194-26/fa17/Lectures/ConvEdgesTemplate.pdf 
