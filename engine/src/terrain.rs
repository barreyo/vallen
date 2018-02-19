
use std::collections::HashMap;

use cgmath::Vector3;

// TODO: Be smarter about this at some point in the future - lookup table somewhere?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Air,
    Grass,
    Snow,
    Water,
    Ice,
    Rock,
}

pub type ChunkIndex = Vector3<u16>;

#[derive(Debug, Copy, Clone)]
pub struct QuantizedFloat {
    pub value: u8,
}

impl QuantizedFloat {
    #[inline(always)]
    pub fn new(val: u8) -> Self {
        QuantizedFloat { value: val }
    }

    #[inline(always)]
    pub fn encode(&mut self, v: f32) {
        self.value = (v * 256.0 + 1.0) as u8;
    }

    #[inline(always)]
    pub fn decode(&self) -> f32 {
        (f32::from(self.value) + 1.0) / 256.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Voxel {
    material: Material,
    occupancy: QuantizedFloat,
}

impl Voxel {
    #[inline]
    pub fn new() -> Self {
        Voxel {
            material: Material::Air,
            occupancy: QuantizedFloat::new(0),
        }
    }

    #[inline(always)]
    pub fn set(&mut self, m: Material, o: QuantizedFloat) {
        self.material = m;
        self.occupancy = o;
    }

    #[inline(always)]
    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    #[inline(always)]
    pub fn set_occupancy(&mut self, o: QuantizedFloat) {
        self.occupancy = o;
    }

    #[inline(always)]
    pub fn set_occupancy_from_f32(&mut self, f: f32) {
        self.occupancy.encode(f);
    }

    #[inline(always)]
    pub fn get_material(&self) -> Material {
        self.material
    }

    #[inline(always)]
    pub fn get_occupancy(&self) -> QuantizedFloat {
        self.occupancy
    }

    #[inline(always)]
    pub fn get_occupancy_as_f32(&self) -> f32 {
        self.occupancy.decode()
    }
}

// TODO: Mipmap
#[derive(Debug, Clone)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    dimension: u16,
}

type VoxelIndex = Vector3<u16>;

impl Chunk {
    /// Empty Chunk
    #[inline(always)]
    pub fn new(dim: u16) -> Chunk {
        let v = (0..(dim * dim * dim)).map(|_| Voxel::new()).collect::<Vec<Voxel>>();
        Chunk {
            voxels: v,
            dimension: dim,
        }
    }

    #[inline(always)]
    fn one_dim_coord(&self, i: VoxelIndex) -> usize {
        (i.x + self.dimension * (i.y + self.dimension * i.z)) as usize
    }

    #[inline(always)]
    pub fn get_voxel_at(&self, idx: VoxelIndex) -> Voxel {
        self.voxels[self.one_dim_coord(idx)]
    }

    #[inline(always)]
    pub fn set_voxel_at(&mut self, idx: VoxelIndex, m: Material, o: QuantizedFloat) {
        let v = {
            let mut temp = self.get_voxel_at(idx);
            temp.set(m, o);
            temp
        };
        let i = self.one_dim_coord(idx);
        self.voxels[i] = v;
    }
}

#[derive(Debug)]
pub struct VoxelGrid {
    chunks: HashMap<ChunkIndex, Chunk>,
}

impl VoxelGrid {
    /// Construct empty voxel grid.
    pub fn new() -> Self {
        VoxelGrid { chunks: HashMap::new() }
    }

    pub fn fill(&mut self, chunk: ChunkIndex) {
        unimplemented!();
    }

    pub fn insert_chunk(&mut self, idx: ChunkIndex, chunk: Chunk) {
        self.chunks.insert(idx, chunk);
    }

    pub fn delete_chunk(&mut self, idx: ChunkIndex) {
        self.chunks.remove(&idx);
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vg_empty() {
        let vg = VoxelGrid::new();
        assert!(vg.is_empty());
    }

    #[test]
    fn ch_set() {
        let mut chunk = Chunk::new(32);
        chunk.set_voxel_at(Vector3::new(1, 5, 3),
                           Material::Grass,
                           QuantizedFloat::new(255));
        assert_eq!(chunk.get_voxel_at(Vector3::new(1, 5, 3)).get_material(),
                   Material::Grass);
    }
}
