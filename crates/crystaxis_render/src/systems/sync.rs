use bevy::prelude::*;
use crystaxis_core::Position;

/// Синхронизирует Transform сущностей с их Position
pub fn sync_transforms(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = pos.0;
    }
}