use std::collections::VecDeque;
use std::marker::PhantomData;

/// Represents a line
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Line<T> {
    /// the first point
    pub x: T,
    /// The second point
    pub y: T,
}

impl<T> Line<T> {
    /// Create a new line using point x and y
    pub fn new(x: T, y: T) -> Self {
        Line { x: x, y: y }
    }
}

/// A polygon with 4 points. Maps to `GL_QUADS`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quad<T> {
    /// the first point of a quad
    pub x: T,
    /// the second point of a quad
    pub y: T,
    /// the third point of a quad
    pub z: T,
    /// the fourth point of a quad
    pub w: T,
}

impl<T> Quad<T> {
    /// create a new `Quad` with supplied vertices
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Quad {
            x: v0,
            y: v1,
            z: v2,
            w: v3,
        }
    }
}

/// A polygon with 3 points. Maps to `GL_TRIANGLE`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triangle<T> {
    /// the first point of a triangle
    pub x: T,
    /// the second point of a triangle
    pub y: T,
    /// the third point of a triangle
    pub z: T,
}

impl<T> Triangle<T> {
    /// create a new `Triangle` with supplied vertcies
    pub fn new(v0: T, v1: T, v2: T) -> Self {
        Triangle {
            x: v0,
            y: v1,
            z: v2,
        }
    }
}

/// An arbitrary-length polygon
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NGon<T> {
    /// the list of vertices of the polygon
    pub verts: VecDeque<T>,
}

impl<T> NGon<T> {
    /// create a new, empty `NGon` polygon
    pub fn new() -> Self {
        NGon {
            verts: VecDeque::new(),
        }
    }

    /// add a vertex to the NGon
    pub fn add_vertex(&mut self, vert: T) {
        self.verts.push_back(vert);
    }
}

/// This is All-the-types container. This exists since some generators
/// produce both `Triangles` and `Quads`, and also to make it possible
/// to be type-generic over the kinds of primitives which generators return
#[derive(Debug, Clone, PartialEq)]
pub enum Polygon<T> {
    /// A wraped triangle
    PolyTri(Triangle<T>),
    /// A wraped quad
    PolyQuad(Quad<T>),
    /// A wrapped arbitrary-length polygon
    PolyNGon(NGon<T>),
}

/// The core mechanism of `Vertices` trait. This is a mechanism for unwraping
/// a polygon extracting all of the vertices that it bound together.
pub trait EmitVertices<T> {
    /// Consume a polygon, each
    /// vertex is emitted to the parent function by calling the supplied
    /// lambda function
    fn emit_vertices<F>(self, F)
    where
        F: FnMut(T);
}

impl<T> EmitVertices<T> for Line<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Line { x, y } = self;
        emit(x);
        emit(y);
    }
}

impl<T> EmitVertices<T> for Triangle<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Triangle { x, y, z } = self;
        emit(x);
        emit(y);
        emit(z);
    }
}

impl<T> EmitVertices<T> for Quad<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Quad { x, y, z, w } = self;
        emit(x);
        emit(y);
        emit(z);
        emit(w);
    }
}

impl<T> EmitVertices<T> for NGon<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        for v in self.verts {
            emit(v);
        }
    }
}

impl<T> EmitVertices<T> for Polygon<T> {
    fn emit_vertices<F>(self, emit: F)
    where
        F: FnMut(T),
    {
        use self::Polygon::{PolyNGon, PolyQuad, PolyTri};

        match self {
            PolyTri(p) => p.emit_vertices(emit),
            PolyQuad(p) => p.emit_vertices(emit),
            PolyNGon(p) => p.emit_vertices(emit),
        }
    }
}

/// An iterator over vertices which have been extracted from a polygon
pub struct VertexIterator<V> {
    buffer: VecDeque<V>,
}

impl<T> Polygon<T> {
    /// extract vertices from the polygon directly, without requiring
    /// an iterator of polygons to transform into an iterator of vertices
    pub fn as_vertices(self) -> VertexIterator<T> {
        let mut buffer = VecDeque::new();
        self.emit_vertices(|v| buffer.push_back(v));
        VertexIterator { buffer: buffer }
    }
}

impl<T> Iterator for VertexIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }
}

/// Supplies a way to convert an iterator of polygons to an iterator
/// of vertices. Useful for when you need to write the vertices into
/// a graphics pipeline.
pub trait Vertices<SRC, V> {
    /// Convert a polygon iterator to a vertices iterator.
    fn vertices(self) -> VertexStreamIterator<SRC, V>;
}

impl<V, P: EmitVertices<V>, T: Iterator<Item = P>> Vertices<T, V> for T {
    fn vertices(self) -> VertexStreamIterator<T, V> {
        VertexStreamIterator {
            source: self,
            buffer: VecDeque::new(),
        }
    }
}

/// an iterator that breaks an iterator of polygons down into
/// an iterator of the individual verticies of those polygons
pub struct VertexStreamIterator<SRC, V> {
    source: SRC,
    buffer: VecDeque<V>,
}

impl<V, U: EmitVertices<V>, SRC: Iterator<Item = U>> Iterator for VertexStreamIterator<SRC, V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => (),
            }

            match self.source.next() {
                Some(p) => p.emit_vertices(|v| self.buffer.push_back(v)),
                None => return None,
            }
        }
    }
}

/// equivalent of `map` but per-vertex
pub trait MapVertex<T, U> {
    /// `Output` should be a a container of the same shape of the type
    /// It's internal values should reflect any transformation the map did.
    type Output;
    /// map a function to each vertex in polygon creating a new polygon
    fn map_vertex<F>(self, F) -> Self::Output
    where
        F: FnMut(T) -> U;
}

impl<T: Clone, U> MapVertex<T, U> for Line<T> {
    type Output = Line<U>;

    fn map_vertex<F>(self, mut map: F) -> Line<U>
    where
        F: FnMut(T) -> U,
    {
        let Line { x, y } = self;
        Line {
            x: map(x),
            y: map(y),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Triangle<T> {
    type Output = Triangle<U>;

    fn map_vertex<F>(self, mut map: F) -> Triangle<U>
    where
        F: FnMut(T) -> U,
    {
        let Triangle { x, y, z } = self;
        Triangle {
            x: map(x),
            y: map(y),
            z: map(z),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Quad<T> {
    type Output = Quad<U>;

    fn map_vertex<F>(self, mut map: F) -> Quad<U>
    where
        F: FnMut(T) -> U,
    {
        let Quad { x, y, z, w } = self;
        Quad {
            x: map(x),
            y: map(y),
            z: map(z),
            w: map(w),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for NGon<T> {
    type Output = NGon<U>;

    fn map_vertex<F>(self, mut map: F) -> NGon<U>
    where
        F: FnMut(T) -> U,
    {
        let mut result = NGon::new();
        for vert in self.verts {
            result.add_vertex(map(vert));
        }
        result
    }
}

impl<T: Clone, U> MapVertex<T, U> for Polygon<T> {
    type Output = Polygon<U>;

    fn map_vertex<F>(self, map: F) -> Polygon<U>
    where
        F: FnMut(T) -> U,
    {
        use self::Polygon::{PolyNGon, PolyQuad, PolyTri};

        match self {
            PolyTri(p) => PolyTri(p.map_vertex(map)),
            PolyQuad(p) => PolyQuad(p.map_vertex(map)),
            PolyNGon(p) => PolyNGon(p.map_vertex(map)),
        }
    }
}

/// This acts very similar to a vertex shader. It gives a way to manipulate
/// and modify the vertices in a polygon. This is useful if you need to scale
/// the mesh using a matrix multiply, or just for modifying the type of each
/// vertex.
pub trait MapToVertices<T, U>: Sized {
    /// `Output` should be a a container of the same shape of the type
    /// It's internal values should reflect any transformation the map did.
    type Output;

    /// from a iterator of polygons, produces a iterator of polygons. Each
    /// vertex in the process is modified with the suppled function.
    fn vertex<F>(self, map: F) -> MapToVerticesIter<Self, T, U, F>
    where
        F: FnMut(T) -> U;
}

impl<VIn, VOut, P, POut: MapVertex<VIn, VOut, Output = P>, T: Iterator<Item = POut>>
    MapToVertices<VIn, VOut> for T
{
    type Output = P;

    fn vertex<F>(self, map: F) -> MapToVerticesIter<T, VIn, VOut, F>
    where
        F: FnMut(VIn) -> VOut,
    {
        MapToVerticesIter {
            src: self,
            f: map,
            phantom: PhantomData,
        }
    }
}

pub struct MapToVerticesIter<SRC, T, U, F: FnMut(T) -> U> {
    src: SRC,
    f: F,
    phantom: PhantomData<(T, U)>,
}

impl<
        'a,
        P,
        POut: MapVertex<T, U, Output = P>,
        SRC: Iterator<Item = POut>,
        T,
        U,
        F: FnMut(T) -> U,
    > Iterator for MapToVerticesIter<SRC, T, U, F>
{
    type Item = P;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.src.size_hint()
    }

    fn next(&mut self) -> Option<P> {
        self.src.next().map(|x| x.map_vertex(|x| (self.f)(x)))
    }
}

/// Convert a Polygon into it's fragments
pub trait EmitLines {
    /// The Vertex defines the corners of a Polygon
    type Vertex;

    /// convert a polygon into lines, each line is emitted via
    /// calling of the callback of `emit` This allow for
    /// a variable amount of lines to be returned
    fn emit_lines<E>(self, emit: E)
    where
        E: FnMut(Line<Self::Vertex>);
}

impl<T: Clone> EmitLines for Triangle<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
    where
        E: FnMut(Line<T>),
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y, self.z.clone()));
        emit(Line::new(self.z, self.x));
    }
}

impl<T: Clone> EmitLines for Quad<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
    where
        E: FnMut(Line<T>),
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y, self.z.clone()));
        emit(Line::new(self.z, self.w.clone()));
        emit(Line::new(self.w, self.x));
    }
}

impl<T: Clone> EmitLines for NGon<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
    where
        E: FnMut(Line<T>),
    {
        debug_assert!(self.verts.len() >= 2);
        let mut iter = self.verts.iter();
        if let Some(mut start) = iter.next() {
            for end in iter {
                emit(Line::new(start.clone(), end.clone()));
                start = end;
            }
        }
    }
}

impl<T: Clone> EmitLines for Polygon<T> {
    type Vertex = T;

    fn emit_lines<E>(self, emit: E)
    where
        E: FnMut(Line<T>),
    {
        use self::Polygon::{PolyNGon, PolyQuad, PolyTri};

        match self {
            PolyTri(x) => x.emit_lines(emit),
            PolyQuad(x) => x.emit_lines(emit),
            PolyNGon(x) => x.emit_lines(emit),
        }
    }
}

/// Creates an LinesIterator from another Iterator
pub trait Lines: Sized {
    /// The type of each point in the lines
    type Vertex;

    /// Convert the iterator into a LinesIterator
    fn lines(self) -> LinesIterator<Self, Self::Vertex>;
}

impl<T, P, V> Lines for T
where
    T: Iterator<Item = P>,
    P: EmitLines<Vertex = V>,
{
    type Vertex = V;

    fn lines(self) -> LinesIterator<T, V> {
        LinesIterator {
            source: self,
            buffer: VecDeque::new(),
        }
    }
}

/// An iterator that turns Polygons into an Iterator of Lines
pub struct LinesIterator<I, V> {
    source: I,
    buffer: VecDeque<Line<V>>,
}

impl<I, P, V> Iterator for LinesIterator<I, V>
where
    I: Iterator<Item = P>,
    P: EmitLines<Vertex = V>,
{
    type Item = Line<V>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, _) = self.source.size_hint();
        (n, None)
    }

    fn next(&mut self) -> Option<Line<V>> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => (),
            }

            match self.source.next() {
                Some(p) => p.emit_lines(|v| self.buffer.push_back(v)),
                None => return None,
            }
        }
    }
}
