use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::core::triangle::Triangle;

use super::vector_3d::Vector3D;

pub fn load_from_obj_file(filename: &str, output: &mut Vec<Triangle>) -> bool {
    let file = File::open(filename).expect("Error reading obj file");
    let buf_reader = BufReader::new(file);
    let mut vertices: Vec<Vector3D> = vec![];
    let mut triangles: Vec<Triangle> = vec![];

    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("v ") {
                    let vertex_data = line.trim_start_matches("v ").trim();
                    match vertex_data.parse::<Vector3D>() {
                        Ok(vertex) => vertices.push(vertex),
                        Err(err) => eprintln!("Failed to parse vertex: {}: {}", vertex_data, err),
                    }
                } else if line.starts_with("f ") {
                    // Parse face (triangle)
                    let triangle_data = line.trim_start_matches("f ").trim();
                    let indices: Vec<&str> = triangle_data.split_whitespace().collect();
                    if indices.len() == 3 {
                        let v1 = parse_index(indices[0]);
                        let v2 = parse_index(indices[1]);
                        let v3 = parse_index(indices[2]);

                        match (v1, v2, v3) {
                            (Some(v1), Some(v2), Some(v3)) => {
                                triangles.push(Triangle::from_vectors(&vertices, [v1, v2, v3]));
                            }
                            _ => eprintln!("Invalid face data: {}", triangle_data),
                        }
                    } else {
                        eprintln!("Face does not have 3 vertices: {}", triangle_data);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return false;
            }
        }
    }

    println!("Loaded {} vertices", vertices.len());
    output.clear();
    output.append(&mut triangles);
    true
}

fn parse_index(s: &str) -> Option<usize> {
    s.split('/')
        .next()? // Only use the vertex index (ignore texture/normal indices)
        .parse::<usize>()
        .ok()
        .map(|i| i - 1) // Convert from 1-based to 0-based index
}
