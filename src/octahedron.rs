use cgmath::{InnerSpace, Vector3};

use generators::{IndexedPolygon, SharedVertex};
use {Polygon, Polygon::PolyTri, Triangle, Vertex};

// from Paul Bourke: http://paulbourke.net/geometry/platonic/
const A: f32 = 1. / (2. * 1.4142135623730951);
const B: f32 = 0.5;

const VERTICES: [[f32; 3]; 6] = [
    [-A, 0., A],
    [A, 0., A],
    [A, 0., -A],
    [-A, 0., -A],
    [0., B, 0.],
    [0., -B, 0.],
];

const FACES: [[usize; 3]; 8] = [
    [3, 0, 4],
    [2, 3, 4],
    [1, 2, 4],
    [0, 1, 4],
    [3, 2, 5],
    [0, 3, 5],
    [2, 1, 5],
    [1, 0, 5],
];

/// a platonic octahedron solid
pub struct Octahedron {
    i: usize,
}

impl Octahedron {
    /// create a simple octahedron with normalized vertices
    pub fn new() -> Self {
        Self { i: 0 }
    }

    fn vert(&self, idx: usize) -> Vertex {
        let position: Vector3<f32> = Vector3::from(VERTICES[idx]).normalize();
        Vertex {
            pos: position.into(),
            normal: position.into(),
        }
    }
}

impl Iterator for Octahedron {
    type Item = Polygon<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (FACES.len(), Some(FACES.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == FACES.len() {
            return None;
        }

        let face = FACES[self.i];
        let va = self.vert(face[0]);
        let vb = self.vert(face[1]);
        let vc = self.vert(face[2]);

        Some(PolyTri(Triangle::new(va, vb, vc)))
    }
}

impl SharedVertex<Vertex> for Octahedron {
    fn shared_vertex_count(&self) -> usize {
        VERTICES.len()
    }

    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vert(idx)
    }
}

impl IndexedPolygon<Polygon<usize>> for Octahedron {
    fn indexed_polygon_count(&self) -> usize {
        FACES.len()
    }

    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let face = FACES[idx];
        PolyTri(Triangle::new(face[0], face[1], face[2]))
    }
}
