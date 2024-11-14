use bevy::gltf::{GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::prelude::*;
use crate::environment::Chunk;

pub fn load_terrain(commands: &mut Commands,
                    chunk: &mut Chunk,
                    meshes: &ResMut<Assets<Mesh>>,
                    materials: &mut ResMut<Assets<StandardMaterial>>,
                    child: &GltfNode,
                    mesh: &GltfMesh,
                    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>
) {
    for primitive in &mesh.primitives {
        let material = primitive.material.clone()
            .unwrap_or_else(|| materials.add(StandardMaterial {
                base_color: Color::srgb_u8(255, 0, 247), ..default()} ));

        let bevy_mesh = primitive.mesh.clone();

        if let Some(col_mesh) = meshes.get(&bevy_mesh) {
            if let Some(collider) = Collider::from_bevy_mesh(col_mesh, &ComputedColliderShape::TriMesh) {
                if chunk.id.is_none() {
                    info!("Parent: {:?}", child);
                    let entity_id = commands.spawn((
                        Name::new(chunk.name.clone()),
                        Transform::default(),
                        GlobalTransform::default(),
                        VisibilityBundle {
                            visibility: Visibility::Visible,
                            ..default()
                        }
                    )).with_children(|commands| {
                        commands.spawn((
                            Name::new(child.name.clone()),
                            PbrBundle {
                            mesh: bevy_mesh,
                            transform: Transform {
                                translation: child.transform.translation,
                                scale: child.transform.scale,
                                ..default()
                            },
                            visibility: Visibility::Visible,
                            material: material.clone(),
                            ..default()
                            },
                            RigidBody::Fixed,
                            collider,
                        ));
                    }).id();

                    chunk.id = Option::from(entity_id);
                    chunk.loaded = true;
                    info!("Loaded {:?}", chunk.name);
                } else {
                    if let Some(entity) = chunk.id {
                        if let Ok((mut visibility, collider_disable)) = visibility_query.get_mut(entity) {
                            *visibility = Visibility::Visible;
                            if collider_disable.is_some() {
                                commands.entity(entity).remove::<ColliderDisabled>();
                            }
                        }
                    }
                    chunk.loaded = true;
                    info!("Loaded {:?}", chunk.name);
                }
            }
        }
    }
}

pub fn load_vegetation(commands: &mut Commands,
                       chunk: &mut Chunk,
                       meshes: &ResMut<Assets<Mesh>>,
                       materials: &mut ResMut<Assets<StandardMaterial>>,
                       child: &GltfNode,
                       mesh: &GltfMesh,
                       visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>
) {
    for primitive in &mesh.primitives {
        let material = primitive.material.clone()
            .unwrap_or_else(|| materials.add(StandardMaterial {
                base_color: Color::srgb_u8(255, 0, 247), ..default()} ));

        let generated_mesh = primitive.mesh.clone();
        info!("inner child: {:?}", child);

        if let Some(entity) = chunk.id {

            commands.entity(entity)
                .with_children(|child_command| {
                child_command.spawn((
                    Name::new(child.name.clone()),
                    PbrBundle {
                        mesh: generated_mesh,
                        transform: Transform {
                            translation: child.transform.translation,
                            scale: child.transform.scale,
                            ..default()
                        },
                        material: material.clone(),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                ));
            });
        } else {
            error!("Load first the load_terrain function for creating a entity for the chunk!");
        }
    }

    info!("Include vegetation for chunk [ {:?} ]", chunk.name.clone());
}

#[allow(unused)]
pub fn load_structures(commands: &mut Commands,
                       chunk: &mut Chunk,
                       meshes: &ResMut<Assets<Mesh>>,
                       materials: &mut ResMut<Assets<StandardMaterial>>,
                       child: &GltfNode,
                       mesh: &GltfMesh,
                       visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>
) {

}