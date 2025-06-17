use bevy::prelude::*;
use crate::components::{
    player::{FollowCamera, Player},
    projectile::{Weapon, Health, HitEffect},
    world::Collidable,
};

pub fn hitscan_shooting(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_q: Query<(&Transform, &mut Weapon), With<Player>>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    collidable_q: Query<&Transform, (With<Collidable>, Without<Player>)>,
    mut target_q: Query<(Entity, &Transform, &mut Health), (Without<Player>, Without<Collidable>)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let camera_transform = if let Ok(transform) = camera_q.single() {
        transform
    } else {
        return;
    };

    for (_player_transform, mut weapon) in player_q.iter_mut() {
        let current_time = time.elapsed_secs();
        
        // Check fire rate
        if current_time - weapon.last_shot_time < 1.0 / weapon.fire_rate {
            continue;
        }
        
        weapon.last_shot_time = current_time;

        // Calculate ray from camera position in camera's forward direction
        let ray_origin = camera_transform.translation;
        let ray_direction = camera_transform.forward().as_vec3();

        // Perform hitscan raycast
        let mut hit_distance = weapon.range;
        let mut hit_point = ray_origin + ray_direction * hit_distance;
        let mut hit_something = false;

        // Check collision with walls
        for wall_transform in collidable_q.iter() {
            if let Some(distance) = ray_intersects_aabb(
                ray_origin,
                ray_direction,
                wall_transform.translation,
                Vec3::new(3.0, 9.0, 3.0), // Wall half-size (6x18x6 units)
            ) {
                if distance < hit_distance && distance > 0.0 {
                    hit_distance = distance;
                    hit_point = ray_origin + ray_direction * distance;
                    hit_something = true;
                }
            }
        }

        // Check collision with other entities (future: other players)
        for (entity, target_transform, mut health) in target_q.iter_mut() {
            if let Some(distance) = ray_intersects_sphere(
                ray_origin,
                ray_direction,
                target_transform.translation,
                1.0, // Target radius
            ) {
                if distance < hit_distance && distance > 0.0 {
                    hit_distance = distance;
                    hit_point = ray_origin + ray_direction * distance;
                    hit_something = true;
                    
                    // Apply damage
                    health.current -= weapon.damage;
                    
                    // Add hit effect
                    commands.entity(entity).insert(HitEffect::default());
                    
                    println!("Hit target! Damage: {}, Health remaining: {}", weapon.damage, health.current);
                }
            }
        }

        // Spawn visual effect at hit point
        spawn_hit_effect(&mut commands, &mut meshes, &mut materials, hit_point, hit_something);
        
        // Play sound effect (placeholder)
        println!("BANG! Shot fired at distance: {:.2}", hit_distance);
    }
}

fn ray_intersects_aabb(
    ray_origin: Vec3,
    ray_direction: Vec3,
    aabb_center: Vec3,
    aabb_half_size: Vec3,
) -> Option<f32> {
    let aabb_min = aabb_center - aabb_half_size;
    let aabb_max = aabb_center + aabb_half_size;
    
    let inv_dir = Vec3::new(
        1.0 / ray_direction.x,
        1.0 / ray_direction.y,
        1.0 / ray_direction.z,
    );
    
    let t1 = (aabb_min - ray_origin) * inv_dir;
    let t2 = (aabb_max - ray_origin) * inv_dir;
    
    let t_min = t1.min(t2);
    let t_max = t1.max(t2);
    
    let t_near = t_min.max_element();
    let t_far = t_max.min_element();
    
    if t_near <= t_far && t_far > 0.0 {
        Some(if t_near > 0.0 { t_near } else { t_far })
    } else {
        None
    }
}

fn ray_intersects_sphere(
    ray_origin: Vec3,
    ray_direction: Vec3,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> Option<f32> {
    let oc = ray_origin - sphere_center;
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * oc.dot(ray_direction);
    let c = oc.dot(oc) - sphere_radius * sphere_radius;
    
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        None
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        
        if t1 > 0.0 {
            Some(t1)
        } else if t2 > 0.0 {
            Some(t2)
        } else {
            None
        }
    }
}

fn spawn_hit_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    hit_wall: bool,
) {
    let color = if hit_wall {
        Color::srgb(1.0, 0.5, 0.0) // Orange for wall hits
    } else {
        Color::srgb(1.0, 1.0, 0.0) // Yellow for max range
    };
    
    // Spawn a small sphere as hit effect
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            emissive: LinearRgba::new(color.to_linear().red, color.to_linear().green, color.to_linear().blue, 1.0),
            ..default()
        })),
        Transform::from_translation(position),
        HitEffect::default(),
    ));
}

pub fn cleanup_hit_effects(
    mut commands: Commands,
    mut hit_effects_q: Query<(Entity, &mut HitEffect)>,
    time: Res<Time>,
) {
    for (entity, mut hit_effect) in hit_effects_q.iter_mut() {
        hit_effect.timer.tick(time.delta());
        
        if hit_effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
