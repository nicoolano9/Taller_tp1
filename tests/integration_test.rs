use tp_1::world::World;

#[test]
fn total_shadows_len_empty_world() {
    let mut world = World::new(45, 5).unwrap();
    assert_eq!(world.total_shadows_len(), 0.0);
}

#[test]
fn total_shadows_len_single_flatlander() {
    let mut world = World::new(45, 5).unwrap();
    world.add_flatlander(0, 2).unwrap();
    let len = world.total_shadows_len();
    assert!(len > 0.0, "La sombra debe tener longitud positiva");
}

#[test]
fn total_shadows_len_multiple_flatlanders() {
    let mut world = World::new(45, 5).unwrap();
    world.add_flatlander(0, 2).unwrap();
    world.add_flatlander(3, 1).unwrap();
    let len = world.total_shadows_len();
    assert!(len > 0.0, "La suma de sombras debe ser positiva");
}

#[test]
fn total_shadows_len_overlapping_shadows() {
    let mut world = World::new(45, 5).unwrap();
    world.add_flatlander(0, 2).unwrap();
    world.add_flatlander(1, 2).unwrap();
    let len = world.total_shadows_len();
    // Si las sombras se superponen, la longitud total no debe ser la suma simple
    let mut world_no_overlap = World::new(45, 5).unwrap();
    world_no_overlap.add_flatlander(0, 2).unwrap();
    world_no_overlap.add_flatlander(10, 2).unwrap();
    let len_no_overlap = world_no_overlap.total_shadows_len();
    assert!(
        len < len_no_overlap,
        "Las sombras superpuestas deben sumar menos que las separadas"
    );
}
