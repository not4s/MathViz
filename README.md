# **MathViz: Interactive Visualization of Complex Analysis and Beyond**

![MathVisualizer Demo](link-to-your-image-or-demo.gif)

Welcome to **MathVisualizer**! This project provides interactive visualizations for complex mathematical concepts, starting with **Complex Analysis and Conformal Mappings**, and is extensible to support other mathematical visualizations such as fractals, differential equations, topology, and more.

## **Table of Contents**

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Modules](#modules)
- [Extensibility](#extensibility)
- [Contributing](#contributing)
- [Future Plans](#future-plans)
- [License](#license)

---

## **Introduction**

**MathVisualizer** is a tool for exploring complex mathematical visualizations using WebGL and WebAssembly, powered by Rust. It currently supports **Interactive Visualizations of Complex Analysis and Conformal Mappings**, with future support planned for various other mathematical domains like fractals, differential equations, and geometry.

The aim is to make math both beautiful and interactive, providing users with a deep visual understanding of complex concepts while maintaining high performance through WebAssembly.

## **Features**

- **Interactive Visualization of Complex Analysis**: 
  - Visualize conformal mappings and complex functions like \( f(z) = z^2 \), \( f(z) = e^z \), and more.
  - See the transformations of various shapes, grids, and domains in the complex plane in real time.
  
- **Modular Architecture**: Easily extendable to other visualizations such as:
  - Fractals (e.g., Mandelbrot and Julia sets).
  - Differential Equations (e.g., phase portraits, solution surfaces).
  - Topology (e.g., Möbius strips, Klein bottles).
  
- **WebGL-powered Rendering**: High-performance, real-time rendering for both 2D and 3D visualizations.
  
- **Dynamic UI**: Adjustable parameters for every visualization, allowing real-time interaction with the underlying mathematics.

## **Installation**

### **Prerequisites**

- **Rust**: Install the latest version of Rust [here](https://www.rust-lang.org/tools/install).
- **wasm-pack**: Install `wasm-pack` to build WebAssembly from Rust:
  
  ```bash
  cargo install wasm-pack
  ```
  
- **Node.js**: To run the local server, make sure Node.js is installed.

### **Steps**
1. **Clone the repository**:
  ```bash
  git clone https://github.com/yourusername/math-visualizer.git
  cd math-visualizer
  ```

2. **Build the Rust WebAssembly package**:
  ```bash
  wasm-pack build --target web
  ```

3. **Install dependencies and serve locally**:
  ```bash
  npm install
  npm run start
  ```

4. **Open the app**: After running the commands, open your browser at `http://localhost:8080` to explore the visualizations.

## **Usage**
Once the app runs, you’ll have a canvas to explore different visualizations. Here’s a quick guide to get started:
1. **Select a Visualization**: Use the sidebar to choose between different mathematical visualizations initially focused on Complex Analysis.
  
2. **Adjust Parameters**: Use the dynamic control panel to modify parameters for the current visualization. This could include:
- Adjusting the zoom level of the complex plane.
- Changing the parameters of a conformal mapping.
- Modifying the resolution and detail level of fractals.
  
3. **Interact**: Use the mouse to zoom, pan, and rotate objects for 3D visualizations. Each visualization may have its unique controls for interaction.
    
4. **Reset View**: Use the reset button in the control panel to return to the default view if you get lost while exploring.

## **Modules**

### **Core Modules**

- **Rendering Engine**: The core module handling WebGL interactions, drawing operations, and event management. All visualizations interact with the canvas through this engine.
  
- **UI Module**: Provides dynamic controls for each type of visualization, ensuring a flexible interface.

### **Visualization Modules**

- **Complex Analysis & Conformal Mappings**: Explore how complex functions transform grids and shapes in the complex plane. This module implements visualizations of \( f(z) = z^2 \), \( f(z) = e^z \), and others.

- **Fractals (Upcoming)**: Visualize Mandelbrot sets, Julia sets, and other recursive fractal structures.

- **Differential Equations (Upcoming)**: Interactive visualizations of solutions to differential equations and their corresponding phase portraits.

## **Extensibility**

This project is designed to be extensible, allowing you to add new mathematical visualizations easily. Each visualization follows a standardized interface (`MathVisualization`) that requires methods for initialization, updating, and rendering.

### **How to Add a New Visualization**

1. **Create a New Module**: Add a new Rust module under `src/visualizations/`.

2. **Implement the `MathVisualization` Trait**:

   ```rust
   pub struct NewVisualization {
       // Fields specific to this visualization
   }

   impl MathVisualization for NewVisualization {
       fn init(&mut self, renderer: &mut Renderer) {
           // Initialize resources
       }

       fn update(&mut self, dt: f32) {
           // Update visualization state
       }

       fn render(&self, renderer: &Renderer) {
           // Render the visualization
       }
   }
   ```

3. **Update the** `VisualizationManager`: Add your new visualization to the visualization manager so users can switch to it from the UI.

4. **Add UI Components**: Define any specific UI controls your new visualization requires (e.g., sliders or buttons).

## **Future Plans**

- **Additional Visualizations**:
  - Fractals and recursive structures (Mandelbrot, Julia sets).
  - Differential Equations and phase portraits.
  - Topology and geometric transformations.

- **Performance Optimization**: Further optimize WebAssembly and WebGL for even smoother performance.

- **Export Features**: Allow users to export visualizations as images or data files for further analysis.

- **Integration with Educational Tools**: Add guided tutorials and explanations within the app to help users learn the mathematics behind the visualizations.

## **License**

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
