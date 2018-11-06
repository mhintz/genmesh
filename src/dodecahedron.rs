use cgmath::{Vector3, InnerSpace};

use generators::{IndexedPolygon, SharedVertex};
use {Polygon, Polygon::PolyNGon, NGon, Vertex};

const PHI: f32 = 1.618033988749895; // (5 ^ 0.5 + 1) * 0.5
const CONJPHI: f32 = 0.6180339887498948; // 1 / PHI

// 20 vertices
const VERTICES: [[f32; 3]; 20] = [
  [-CONJPHI, -PHI, 0.], // v1
  [CONJPHI, -PHI, 0.], // v2
  [1., -1., 1.], // v3
  [0., -CONJPHI, PHI], // v4
  [-1., -1., 1.], // v5
  [-PHI, 0., CONJPHI], // v6
  [-1., -1., -1.], // v7
  [1., -1., -1.], // v8
  [PHI, 0., CONJPHI], // v9
  [0., CONJPHI, PHI], // v10
  [-1., 1., 1.], // v11.
  [-PHI, 0., -CONJPHI], // v12
  [0., -CONJPHI, -PHI], // v13
  [PHI, 0., -CONJPHI], // v14
  [1., 1., 1.], // v15
  [CONJPHI, PHI, 0.], // v16
  [-CONJPHI, PHI, 0.], // v17
  [-1., 1., -1.], // v18
  [0., CONJPHI, -PHI], // v19
  [1., 1., -1.], // v20
];

// 12 pentagonal faces
const FACES: [[usize; 5]; 12] = [
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

/// Platonic dodecahedron, made of pentagons
pub struct Dodecahedron {
    i: usize,
}

impl Dodecahedron {
    /// Create a unit Dodecahedron
    pub fn new() -> Self {
        Self {
            i: 0,
        }
    }

    fn vert(&self, index: usize) -> Vertex {
        let un_normalized: Vector3<f32> = VERTICES[index].into();
        let position = un_normalized.normalize();
        let normal = position;
        Vertex {
            pos: position.into(),
            normal: normal.into(),
        }
    }
}

impl Iterator for Dodecahedron {
    type Item = Polygon<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (FACES.len(), Some(FACES.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == FACES.len() {
            return None;
        }

        let mut result = NGon::new();

        let face = FACES[self.i];
        result.add_vertex(self.vert(face[0]));
        result.add_vertex(self.vert(face[1]));
        result.add_vertex(self.vert(face[2]));
        result.add_vertex(self.vert(face[3]));
        result.add_vertex(self.vert(face[4]));
        self.i += 1;

        Some(PolyNGon(result))
    }
}

impl SharedVertex<Vertex> for Dodecahedron {
    fn shared_vertex_count(&self) -> usize {
        VERTICES.len()
    }

    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vert(idx)
    }
}

impl IndexedPolygon<Polygon<usize>> for Dodecahedron {
    fn indexed_polygon_count(&self) -> usize {
        FACES.len()
    }

    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let mut result = NGon::new();

        let face = FACES[idx];
        result.add_vertex(face[0]);
        result.add_vertex(face[1]);
        result.add_vertex(face[2]);
        result.add_vertex(face[3]);
        result.add_vertex(face[4]);

        PolyNGon(result)
    }
}

