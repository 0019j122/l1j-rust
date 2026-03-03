/// Grid-based spatial partitioning for efficient visibility queries.
///
/// Each map is divided into 32x32 tile regions. When querying visible
/// objects, only the current region and its 8 neighbors are checked,
/// reducing complexity from O(n) to O(k) where k = objects in nearby regions.

use std::collections::{HashMap, HashSet};

/// Region size in tiles. Each region covers 32x32 tiles.
pub const REGION_SIZE: i32 = 32;

/// L1J screen visibility is roughly 18 tiles in each direction.
/// With 32x32 regions, checking current + 8 neighbors covers this.
pub const SCREEN_RANGE: i32 = 18;

/// A region key: (map_id, region_x, region_y).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionKey {
    pub map_id: i32,
    pub rx: i32,
    pub ry: i32,
}

impl RegionKey {
    /// Compute the region key for a world position.
    pub fn from_world(map_id: i32, x: i32, y: i32) -> Self {
        RegionKey {
            map_id,
            rx: x.div_euclid(REGION_SIZE),
            ry: y.div_euclid(REGION_SIZE),
        }
    }

    /// Get the 9 neighboring region keys (including self).
    pub fn neighbors(&self) -> [RegionKey; 9] {
        [
            RegionKey { map_id: self.map_id, rx: self.rx - 1, ry: self.ry - 1 },
            RegionKey { map_id: self.map_id, rx: self.rx,     ry: self.ry - 1 },
            RegionKey { map_id: self.map_id, rx: self.rx + 1, ry: self.ry - 1 },
            RegionKey { map_id: self.map_id, rx: self.rx - 1, ry: self.ry     },
            *self, // center
            RegionKey { map_id: self.map_id, rx: self.rx + 1, ry: self.ry     },
            RegionKey { map_id: self.map_id, rx: self.rx - 1, ry: self.ry + 1 },
            RegionKey { map_id: self.map_id, rx: self.rx,     ry: self.ry + 1 },
            RegionKey { map_id: self.map_id, rx: self.rx + 1, ry: self.ry + 1 },
        ]
    }
}

/// Object ID type.
pub type ObjectId = i32;

/// The grid index: maps RegionKey -> set of object IDs in that region.
///
/// This is the core spatial index that makes 10,000 NPC visibility
/// queries fast. Instead of iterating all objects on the map,
/// we only iterate objects in the 9 surrounding regions.
pub struct WorldGrid {
    regions: HashMap<RegionKey, HashSet<ObjectId>>,
}

impl WorldGrid {
    pub fn new() -> Self {
        WorldGrid {
            regions: HashMap::new(),
        }
    }

    /// Add an object to the grid at the given world position.
    pub fn add(&mut self, id: ObjectId, map_id: i32, x: i32, y: i32) {
        let key = RegionKey::from_world(map_id, x, y);
        self.regions.entry(key).or_default().insert(id);
    }

    /// Remove an object from the grid at its current world position.
    pub fn remove(&mut self, id: ObjectId, map_id: i32, x: i32, y: i32) {
        let key = RegionKey::from_world(map_id, x, y);
        if let Some(set) = self.regions.get_mut(&key) {
            set.remove(&id);
            if set.is_empty() {
                self.regions.remove(&key);
            }
        }
    }

    /// Move an object from one position to another.
    ///
    /// Only updates the grid if the object moved to a different region.
    pub fn move_object(
        &mut self,
        id: ObjectId,
        map_id: i32,
        old_x: i32,
        old_y: i32,
        new_x: i32,
        new_y: i32,
    ) {
        let old_key = RegionKey::from_world(map_id, old_x, old_y);
        let new_key = RegionKey::from_world(map_id, new_x, new_y);

        if old_key != new_key {
            // Remove from old region
            if let Some(set) = self.regions.get_mut(&old_key) {
                set.remove(&id);
                if set.is_empty() {
                    self.regions.remove(&old_key);
                }
            }
            // Add to new region
            self.regions.entry(new_key).or_default().insert(id);
        }
    }

    /// Get all object IDs in the 9 surrounding regions of a position.
    ///
    /// This is the main visibility query - O(k) where k = nearby objects.
    pub fn get_nearby(&self, map_id: i32, x: i32, y: i32) -> Vec<ObjectId> {
        let center = RegionKey::from_world(map_id, x, y);
        let mut result = Vec::new();

        for key in &center.neighbors() {
            if let Some(set) = self.regions.get(key) {
                result.extend(set.iter());
            }
        }

        result
    }

    /// Get total number of tracked objects.
    pub fn total_objects(&self) -> usize {
        self.regions.values().map(|s| s.len()).sum()
    }

    /// Get number of active regions.
    pub fn active_regions(&self) -> usize {
        self.regions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_key_from_world() {
        let key = RegionKey::from_world(4, 32800, 32850);
        assert_eq!(key.rx, 32800 / 32);
        assert_eq!(key.ry, 32850 / 32);
        assert_eq!(key.map_id, 4);
    }

    #[test]
    fn test_same_region_objects() {
        let mut grid = WorldGrid::new();
        grid.add(1, 4, 100, 200);
        grid.add(2, 4, 105, 205);
        grid.add(3, 4, 110, 210);

        let nearby = grid.get_nearby(4, 100, 200);
        assert!(nearby.contains(&1));
        assert!(nearby.contains(&2));
        assert!(nearby.contains(&3));
    }

    #[test]
    fn test_different_map_isolation() {
        let mut grid = WorldGrid::new();
        grid.add(1, 4, 100, 200);
        grid.add(2, 5, 100, 200); // same position, different map

        let nearby_map4 = grid.get_nearby(4, 100, 200);
        let nearby_map5 = grid.get_nearby(5, 100, 200);

        assert!(nearby_map4.contains(&1));
        assert!(!nearby_map4.contains(&2));
        assert!(nearby_map5.contains(&2));
        assert!(!nearby_map5.contains(&1));
    }

    #[test]
    fn test_far_away_not_visible() {
        let mut grid = WorldGrid::new();
        grid.add(1, 4, 100, 200);
        grid.add(2, 4, 500, 500); // far away

        let nearby = grid.get_nearby(4, 100, 200);
        assert!(nearby.contains(&1));
        assert!(!nearby.contains(&2));
    }

    #[test]
    fn test_move_changes_region() {
        let mut grid = WorldGrid::new();
        grid.add(1, 4, 100, 200);

        // Move to a different region
        grid.move_object(1, 4, 100, 200, 500, 500);

        let nearby_old = grid.get_nearby(4, 100, 200);
        let nearby_new = grid.get_nearby(4, 500, 500);

        assert!(!nearby_old.contains(&1));
        assert!(nearby_new.contains(&1));
    }

    #[test]
    fn test_remove_object() {
        let mut grid = WorldGrid::new();
        grid.add(1, 4, 100, 200);
        assert_eq!(grid.total_objects(), 1);

        grid.remove(1, 4, 100, 200);
        assert_eq!(grid.total_objects(), 0);
    }

    #[test]
    fn test_10000_npcs_performance() {
        let mut grid = WorldGrid::new();

        // Spawn 10,000 NPCs across the map
        for i in 0..10_000i32 {
            let x = 32000 + (i as i32 % 200);
            let y = 32000 + (i as i32 / 200);
            grid.add(i, 4, x, y);
        }

        assert_eq!(grid.total_objects(), 10_000);

        // Query nearby objects from the center
        let nearby = grid.get_nearby(4, 32100, 32025);

        // Should only return objects in 9 nearby regions, NOT all 10,000
        assert!(nearby.len() < 10_000);
        assert!(nearby.len() > 0);
    }
}
