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
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
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
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
                // NORTH
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                    ],
                },
                // EAST
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0,
                        },
                    ],
                },
                // WEST
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
                // TOP
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        },
                    ],
                },
                // BOTTOM
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ],
                },
            ],
        }
    }

    pub fn from_pyramid() -> Mesh {
        Mesh {
            triangles: vec![
                // BASE (using two triangles to form a square base)
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Bottom-left of the base
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Top-left of the base
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Bottom-right of the base
                    ],
                },
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Top-left of the base
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Top-right of the base
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Bottom-right of the base
                    ],
                },
                // SIDE TRIANGLES (each connects the apex to the base)
                // South Triangle
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Base bottom-left
                        Vector3D {
                            x: 0.7,
                            y: 0.7,
                            z: 1.0,
                        }, // Apex of the pyramid
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Base bottom-right
                    ],
                },
                // West Triangle
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Base bottom-left
                        Vector3D {
                            x: 0.7,
                            y: 0.7,
                            z: 1.0,
                        }, // Apex of the pyramid
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Base top-left
                    ],
                },
                // East Triangle
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        }, // Base bottom-right
                        Vector3D {
                            x: 0.7,
                            y: 0.7,
                            z: 1.0,
                        }, // Apex of the pyramid
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Base top-right
                    ],
                },
                // North Triangle
                Triangle {
                    vector3d: [
                        Vector3D {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Base top-left
                        Vector3D {
                            x: 0.7,
                            y: 0.7,
                            z: 1.0,
                        }, // Apex of the pyramid
                        Vector3D {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0,
                        }, // Base top-right
                    ],
                },
            ],
        }
    }

    pub fn from_cilinder(num_segments: u32) -> Mesh {
        let mut triangles = Vec::new();

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let top_center = Vector3D {
                x: 0.7,
                y: 0.7,
                z: 1.0,
            };
            let top1 = Vector3D {
                x: 0.7 + 0.7 * angle1.cos(),
                y: 0.7 + 0.7 * angle1.sin(),
                z: 1.0,
            };
            let top2 = Vector3D {
                x: 0.7 + 0.7 * angle2.cos(),
                y: 0.7 + 0.7 * angle2.sin(),
                z: 1.0,
            };

            triangles.push(Triangle {
                vector3d: [top_center, top1, top2],
            });
        }

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let bottom_center = Vector3D {
                x: 0.7,
                y: 0.7,
                z: 0.0,
            };
            let bottom1 = Vector3D {
                x: 0.7 + 0.7 * angle1.cos(),
                y: 0.7 + 0.7 * angle1.sin(),
                z: 0.0,
            };
            let bottom2 = Vector3D {
                x: 0.7 + 0.7 * angle2.cos(),
                y: 0.7 + 0.7 * angle2.sin(),
                z: 0.0,
            };

            triangles.push(Triangle {
                vector3d: [bottom_center, bottom1, bottom2],
            });
        }

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let top1 = Vector3D {
                x: 0.7 + 0.7 * angle1.cos(),
                y: 0.7 + 0.7 * angle1.sin(),
                z: 1.0,
            };
            let top2 = Vector3D {
                x: 0.7 + 0.7 * angle2.cos(),
                y: 0.7 + 0.7 * angle2.sin(),
                z: 1.0,
            };
            let bottom1 = Vector3D {
                x: 0.7 + 0.7 * angle1.cos(),
                y: 0.7 + 0.7 * angle1.sin(),
                z: 0.0,
            };
            let bottom2 = Vector3D {
                x: 0.7 + 0.7 * angle2.cos(),
                y: 0.7 + 0.7 * angle2.sin(),
                z: 0.0,
            };

            triangles.push(Triangle {
                vector3d: [top1.clone(), top2.clone(), bottom1.clone()],
            });
            triangles.push(Triangle {
                vector3d: [top2.clone(), bottom2.clone(), bottom1.clone()],
            });
        }
        Mesh { triangles }
    }
}
