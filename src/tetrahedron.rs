use cgmath::{InnerSpace, Vector3};

use generators::{IndexedPolygon, SharedVertex};
use {Polygon, Polygon::PolyTri, Triangle, Vertex};

// from Paul Bourke: http://paulbourke.net/geometry/platonic/
const VERTICES: [[f32; 3]; 4] = [[1., 1., 1.], [1., -1., -1.], [-1., 1., -1.], [-1., -1., 1.]];

const FACES: [[usize; 3]; 4] = [[0, 2, 1], [2, 3, 1], [0, 1, 3], [0, 3, 2]];

// alternative tetrahedron vertices implementation from pex: http://vorg.github.io/pex/docs
// const ROOT_3: f32 = 1.7320508075688772;  // 3 ^ 0.5
// const ROOT_6: f32 = 2.449489742783178; // 6 ^ 0.5

// const VERTICES: [[f32; 3]; 4] = [
//     [ROOT_3 / 3., -ROOT_6 / 3. * 0.333 + ROOT_6 * 0.025, 0.],
//     [-ROOT_3 / 6., -ROOT_6 / 3. * 0.333 + ROOT_6 * 0.025, 0.5],
//     [-ROOT_3 / 6., -ROOT_6 / 3. * 0.333 + ROOT_6 * 0.025, -0.5],
//     [0.,  ROOT_6 / 3. * 0.666 + ROOT_6 * 0.025, 0.],
// ];

// const FACES: [[usize; 3]; 4] = [
//     [0, 1, 2],
//     [3, 1, 0],
//     [3, 0, 2],
//     [3, 2, 1],
// ];

/// a platonic tetrahedron solid
pub struct Tetrahedron {
    i: usize,
}

impl Tetrahedron {
    /// create a simple tetrahedron with normalized vertices
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

impl Iterator for Tetrahedron {
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

impl SharedVertex<Vertex> for Tetrahedron {
    fn shared_vertex_count(&self) -> usize {
        VERTICES.len()
    }

    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vert(idx)
    }
}

impl IndexedPolygon<Polygon<usize>> for Tetrahedron {
    fn indexed_polygon_count(&self) -> usize {
        FACES.len()
    }

    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let face = FACES[idx];
        PolyTri(Triangle::new(face[0], face[1], face[2]))
    }
}
