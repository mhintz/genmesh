use std::collections::VecDeque;

use Polygon::{PolyQuad, PolyTri, PolyNGon};
use {Polygon, Quad, Triangle, NGon};

/// provides a way to convert a polygon down to triangles
pub trait EmitTriangles {
    /// The content of each point in the face
    type Vertex;

    /// convert a polygon to one or more triangles, each triangle
    /// is returned by calling `emit`
    fn emit_triangles<F>(&self, F)
    where
        F: FnMut(Triangle<Self::Vertex>);
}

impl<T: Clone> EmitTriangles for Quad<T> {
    type Vertex = T;

    /// triangulates a quad
    fn emit_triangles<F>(&self, mut emit: F)
    where
        F: FnMut(Triangle<T>),
    {
        let &Quad {
            ref x,
            ref y,
            ref z,
            ref w,
        } = self;
        emit(Triangle::new(x.clone(), y.clone(), z.clone()));
        emit(Triangle::new(z.clone(), w.clone(), x.clone()));
    }
}

impl<T: Clone> EmitTriangles for Triangle<T> {
    type Vertex = T;

    /// triangulates a triangle
    fn emit_triangles<F>(&self, mut emit: F)
    where
        F: FnMut(Triangle<T>),
    {
        emit(self.clone());
    }
}

impl<T: Clone> EmitTriangles for NGon<T> {
    type Vertex = T;

    /// triangulates a convex n-sided polygon
    fn emit_triangles<F>(&self, mut emit: F)
    where
        F: FnMut(Triangle<T>),
    {
        debug_assert!(self.verts.len() >= 3);

        let mut v_iter = self.verts.iter();
        if let Some(start) = v_iter.next() {
            if let Some(mut recent) = v_iter.next() {
                while let Some(vert) = v_iter.next() {
                    emit(Triangle::new(start.clone(), recent.clone(), vert.clone()));
                    recent = vert;
                }
            }
        }
    }
}

impl<T: Clone> EmitTriangles for Polygon<T> {
    type Vertex = T;

    /// triangulation, generic over polygon types
    fn emit_triangles<F>(&self, emit: F)
    where
        F: FnMut(Triangle<T>),
    {
        match self {
            &PolyTri(ref t) => t.emit_triangles(emit),
            &PolyQuad(ref q) => q.emit_triangles(emit),
            &PolyNGon(ref n) => n.emit_triangles(emit),
        }
    }
}

/// `Triangluate` is a easy to to convert any Polygon stream to
/// a stream of triangles. This is useful since Quads and other geometry
/// are not supported by modern graphics pipelines like OpenGL.
pub trait Triangulate<T, V> {
    /// convert a stream of Polygons to a stream of triangles
    fn triangulate(self) -> TriangulateIterator<T, V>;
}

impl<V, P: EmitTriangles<Vertex = V>, T: Iterator<Item = P>> Triangulate<T, V> for T {
    fn triangulate(self) -> TriangulateIterator<T, V> {
        TriangulateIterator::new(self)
    }
}

/// Used to iterator of polygons into a iterator of triangles
pub struct TriangulateIterator<SRC, V> {
    source: SRC,
    buffer: VecDeque<Triangle<V>>,
}

impl<V, U: EmitTriangles<Vertex = V>, SRC: Iterator<Item = U>> TriangulateIterator<SRC, V> {
    fn new(src: SRC) -> TriangulateIterator<SRC, V> {
        TriangulateIterator {
            source: src,
            buffer: VecDeque::new(),
        }
    }
}

impl<V, U: EmitTriangles<Vertex = V>, SRC: Iterator<Item = U>> Iterator
    for TriangulateIterator<SRC, V>
{
    type Item = Triangle<V>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, _) = self.source.size_hint();
        (n, None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => (),
            }

            match self.source.next() {
                Some(p) => p.emit_triangles(|v| self.buffer.push_back(v)),
                None => return None,
            }
        }
    }
}
