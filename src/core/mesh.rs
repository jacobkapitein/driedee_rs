use super::{triangle::Triangle, vector_3d::Vector3D};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_cube() -> Mesh {
        Mesh {
            triangles: vec![
                // SOUTH
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 0.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
                // NORTH
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 1.4,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                    ],
                },
                // EAST
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 1.4,
                        },
                    ],
                },
                // WEST
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 0.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 1.4,
                        },
                    ],
                },
                // TOP
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.4,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 1.4,
                            z: 0.0,
                        },
                    ],
                },
                // BOTTOM
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 1.4,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.4,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.4,
                            y: 0.0,
                            z: 1.4,
                        },
                    ],
                },
            ],
        }
    }
}
