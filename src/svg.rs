use crate::BinaryImage;
use std::collections::BTreeSet;

pub struct SvgEncoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    start: Coord,
    end: Coord,
}

impl Edge {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            start: Coord { x: x1, y: y1 },
            end: Coord { x: x2, y: y2 },
        }
    }
}

impl SvgEncoder {
    /// Renders a binary image as an SVG string.
    pub fn render(image: &impl BinaryImage) -> String {
        let mut output = String::new();
        Self::write_to(&mut output, image).unwrap();
        output
    }

    /// Writes the SVG representation of a binary image to a writer.
    pub fn write_to<W: std::fmt::Write>(out: &mut W, image: &impl BinaryImage) -> std::fmt::Result {
        let width = image.width();
        let height = image.height();

        writeln!(
            out,
            "<svg \
                xmlns=\"http://www.w3.org/2000/svg\" \
                width=\"128\" \
                height=\"128\" \
                viewBox=\"0 0 {width} {height}\"\
            >"
        )?;

        writeln!(
            out,
            "<rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"white\"/>"
        )?;

        Self::writeln_path_element(out, image)?;

        writeln!(out, "</svg>")
    }

    /// Writes the path element for the binary image to a writer.
    fn writeln_path_element<W: std::fmt::Write>(
        out: &mut W,
        image: &impl BinaryImage,
    ) -> std::fmt::Result {
        write!(out, "<path d=\"")?;

        let paths = Self::extract_path(image);
        let mut current = Coord { x: 0, y: 0 };

        for path in &paths {
            Self::write_closed_subpath(out, current, path)?;
            current = path[0];
        }

        writeln!(
            out,
            "\" fill=\"black\" stroke=\"none\" stroke-width=\"0\"/>"
        )
    }

    /// Writes a closed subpath to a writer, starting from the current coordinate.
    fn write_closed_subpath<W: std::fmt::Write>(
        out: &mut W,
        current: Coord,
        path: &[Coord],
    ) -> std::fmt::Result {
        let mut current = current;

        for (i, coord) in path.iter().enumerate() {
            let (dx, dy) = Self::relative_move(current, *coord);
            if i == 0 {
                write!(out, "m{dx} {dy}")?;
            } else if dx == 0 {
                write!(out, "v{dy}")?;
            } else if dy == 0 {
                write!(out, "h{dx}")?;
            } else {
                write!(out, "l{dx} {dy}")?;
            }
            current = *coord;
        }

        write!(out, "z")
    }

    /// Calculates the relative move from one coordinate to another.
    fn relative_move(from: Coord, to: Coord) -> (isize, isize) {
        // The maximum coordinate value is the sum of the maximum number of QR code cells (177)
        // and the quiet zone size (4) on both sides; however, to ensure a safety margin,
        // verify that the value is 255 or less.
        debug_assert!(from.x <= 255);
        debug_assert!(from.y <= 255);
        debug_assert!(to.x <= 255);
        debug_assert!(to.y <= 255);

        let dx = to.x as isize - from.x as isize;
        let dy = to.y as isize - from.y as isize;
        (dx, dy)
    }

    /// Extracts all closed paths from a binary image.
    fn extract_path(image: &impl BinaryImage) -> Vec<Vec<Coord>> {
        let mut edges = Self::create_edges(image);
        let mut all_paths = Vec::new();

        while !edges.is_empty() {
            let path = Self::extract_closed_subpath(&mut edges);
            all_paths.push(path);
        }

        all_paths
    }

    /// Creates a set of edges representing the boundaries of the black pixels in a binary image.
    fn create_edges(image: &impl BinaryImage) -> BTreeSet<Edge> {
        let width = image.width();
        let height = image.height();
        let mut edges = BTreeSet::new();
        for y in 0..height {
            for x in 0..width {
                if !image.get(x, y) {
                    continue;
                }
                if (x > 0 && !image.get(x - 1, y)) || x == 0 {
                    edges.insert(Edge::new(x, y, x, y + 1));
                }
                if y + 1 >= height || !image.get(x, y + 1) {
                    edges.insert(Edge::new(x, y + 1, x + 1, y + 1));
                }
                if x + 1 >= width || !image.get(x + 1, y) {
                    edges.insert(Edge::new(x + 1, y + 1, x + 1, y));
                }
                if (y > 0 && !image.get(x, y - 1)) || y == 0 {
                    edges.insert(Edge::new(x + 1, y, x, y));
                }
            }
        }
        edges
    }

    /// Extracts a closed subpath from the set of edges, starting from an arbitrary edge.
    fn extract_closed_subpath(edges: &mut BTreeSet<Edge>) -> Vec<Coord> {
        debug_assert!(!edges.is_empty(), "edges should not be empty");

        let start_edge = edges.first().expect("edges should not be empty");
        let mut path = Self::extract_subpath_fragment(edges, start_edge.start);

        while let Some(index) = Self::find_index_of_cross_point(&path, edges) {
            let cross_point = path[index];
            let fragment = Self::extract_subpath_fragment(edges, cross_point);
            path.splice(index..index, fragment);
        }

        if path.len() < 3 {
            return path;
        }

        // simplify the path by removing points that are not corners
        (0..path.len())
            .filter(|&i| {
                if i == 0 {
                    true
                } else {
                    let prev = &path[i - 1];
                    let next = &path[(i + 1) % path.len()];
                    prev.x != next.x && prev.y != next.y
                }
            })
            .map(|i| path[i])
            .collect()
    }

    /// Extracts a subpath fragment from the set of edges, starting from a given coordinate.
    fn extract_subpath_fragment(edges: &mut BTreeSet<Edge>, start: Coord) -> Vec<Coord> {
        let mut fragment = Vec::new();
        let mut current = start;
        while let Some(edge) = Self::find_edge_starting_from(edges, current) {
            edges.remove(&edge);
            fragment.push(edge.start);
            current = edge.end;
        }
        fragment
    }

    /// Finds an edge in the set of edges that starts from a given coordinate.
    fn find_edge_starting_from(edges: &BTreeSet<Edge>, start: Coord) -> Option<Edge> {
        let start_key = Edge::new(start.x, start.y, 0, 0);
        edges.range(start_key..).next().and_then(|edge| {
            if edge.start == start {
                Some(*edge)
            } else {
                None
            }
        })
    }

    /// Finds the index of a coordinate in a path that has an outgoing edge in the set of edges.
    fn find_index_of_cross_point(path: &[Coord], edges: &BTreeSet<Edge>) -> Option<usize> {
        for (i, &coord) in path.iter().enumerate() {
            if Self::find_edge_starting_from(edges, coord).is_some() {
                return Some(i);
            }
        }
        None
    }
}
