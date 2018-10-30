use cgmath::{InnerSpace, Vector3};

use generators::{IndexedPolygon, SharedVertex};
use {NGon, Vertex};

const PHI = 1.618033988749895; // (5 ^ 0.5 + 1) * 0.5
const CONJPHI = 0.6180339887498948; // 1 / PHI

// 20 vertices
const VERTICES: [[f32; 3]; 20] = [
  [-conjphi, -phi, 0], // v1
  [conjphi, -phi, 0], // v2
  [1, -1, 1], // v3
  [0, -conjphi, phi], // v4
  [-1, -1, 1], // v5
  [-phi, 0, conjphi], // v6
  [-1, -1, -1], // v7
  [1, -1, -1], // v8
  [phi, 0, conjphi], // v9
  [0, conjphi, phi], // v10
  [-1, 1, 1], // v11
  [-phi, 0, -conjphi], // v12
  [0, -conjphi, -phi], // v13
  [phi, 0, -conjphi], // v14
  [1, 1, 1], // v15
  [conjphi, phi, 0], // v16
  [-conjphi, phi, 0], // v17
  [-1, 1, -1], // v18
  [0, conjphi, -phi], // v19
  [1, 1, -1], // v20
];

// 12 pentagonal faces
const FACES: [[f32; 5]; 12] = [
    [0, 1, 2, 3, 4],
    [1, 0, 6, 12, 7],
    [2, 1, 7, 13, 8],
    [3, 2, 8, 14, 9],
    [4, 3, 9, 10, 5],
    [0, 4, 5, 11, 6],
    [17, 18, 12, 6, 11],
    [18, 19, 13, 7, 12],
    [19, 15, 14, 8, 13],
    [15, 16, 10, 9, 14],
    [16, 17, 11, 5, 10],
    [15, 19, 18, 17, 16]
];

pub struct Dodecahedron {
    
}

