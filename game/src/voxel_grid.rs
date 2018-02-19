use std::collections::HashMap;

use cgmath::Vector3;

#[cfg(test)]
use quickcheck::Arbitrary;
#[cfg(test)]
use quickcheck::Gen;

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
    #[inline]
    pub fn new(val: u8) -> Self {
        QuantizedFloat { value: val }
    }

    #[inline]
    pub fn encode(&mut self, v: f32) {
        self.value = (v * 256.0 + 1.0) as u8;
    }

    #[inline]
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

    pub fn new_with_args(mat: Material, occ: QuantizedFloat) -> Self {
        Voxel {
            material: mat,
            occupancy: occ,
        }
    }

    #[inline]
    pub fn set(&mut self, m: Material, o: QuantizedFloat) {
        self.material = m;
        self.occupancy = o;
    }

    #[inline]
    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    #[inline]
    pub fn set_occupancy(&mut self, o: QuantizedFloat) {
        self.occupancy = o;
    }

    #[inline]
    pub fn set_occupancy_from_f32(&mut self, f: f32) {
        self.occupancy.encode(f);
    }

    #[inline]
    pub fn get_material(&self) -> Material {
        self.material
    }

    #[inline]
    pub fn get_occupancy(&self) -> QuantizedFloat {
        self.occupancy
    }

    #[inline]
    pub fn get_occupancy_as_f32(&self) -> f32 {
        self.occupancy.decode()
    }
}

// TODO: Mipmap
#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: Vec<Voxel>,
    dimension: u16,
}

type VoxelIndex = Vector3<u16>;

impl Chunk {
    /// Empty Chunk
    #[inline]
    pub fn new(dim: u16) -> Chunk {
        let v = (0..(dim * dim * dim))
            .map(|_| Voxel::new())
            .collect::<Vec<Voxel>>();
        Chunk {
            voxels: v,
            dimension: dim,
        }
    }

    #[inline]
    fn one_dim_coord(&self, i: VoxelIndex) -> usize {
        (i.x + self.dimension * (i.y + self.dimension * i.z)) as usize
    }

    #[inline]
    pub fn get_voxel_at(&self, idx: VoxelIndex) -> Voxel {
        self.voxels[self.one_dim_coord(idx)]
    }

    #[inline]
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

#[derive(Debug, Clone)]
pub struct VoxelGrid {
    chunks: HashMap<ChunkIndex, Chunk>,
}

impl VoxelGrid {
    /// Construct empty voxel grid.
    pub fn new() -> Self {
        VoxelGrid {
            chunks: HashMap::new(),
        }
    }

    pub fn fill(&mut self, _chunk: ChunkIndex) {
        unimplemented!();
    }

    pub fn insert_chunk(&mut self, idx: &ChunkIndex, chunk: Chunk) {
        self.chunks.insert(*idx, chunk);
    }

    pub fn delete_chunk(&mut self, idx: &ChunkIndex) {
        self.chunks.remove(idx);
    }

    pub fn neighbors(&self, idx: &ChunkIndex) -> [Option<&Chunk>; 6] {
        [
            self.chunks.get(&Vector3::new(idx.x + 1, idx.y, idx.z)),
            self.chunks.get(&Vector3::new(idx.x - 1, idx.y, idx.z)),
            self.chunks.get(&Vector3::new(idx.x, idx.y + 1, idx.z)),
            self.chunks.get(&Vector3::new(idx.x, idx.y - 1, idx.z)),
            self.chunks.get(&Vector3::new(idx.x, idx.y, idx.z + 1)),
            self.chunks.get(&Vector3::new(idx.x, idx.y, idx.z - 1)),
        ]
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

// ---
// Arbitrary implementation for QuickCheck prop testing.
// ---

#[cfg(test)]
impl Arbitrary for Material {
    fn arbitrary<G: Gen>(g: &mut G) -> Material {
        let v = &[
            Material::Air,
            Material::Grass,
            Material::Snow,
            Material::Water,
            Material::Ice,
            Material::Rock,
        ];
        *g.choose(v).unwrap()
    }
}

#[cfg(test)]
impl Arbitrary for QuantizedFloat {
    fn arbitrary<G: Gen>(g: &mut G) -> QuantizedFloat {
        QuantizedFloat::new(Arbitrary::arbitrary(g))
    }
}

#[cfg(test)]
impl Arbitrary for Voxel {
    fn arbitrary<G: Gen>(g: &mut G) -> Voxel {
        Voxel::new_with_args(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
    }
}

#[cfg(test)]
impl Arbitrary for Chunk {
    fn arbitrary<G: Gen>(g: &mut G) -> Chunk {
        let size: u16 = Arbitrary::arbitrary(g);
        let mut ch = Chunk::new(size);
        for v in &mut ch.voxels {
            v.set(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
        }
        ch
    }
}

#[cfg(test)]
impl Arbitrary for VoxelGrid {
    fn arbitrary<G: Gen>(g: &mut G) -> VoxelGrid {
        let mut grid = VoxelGrid::new();
        let size = {
            let s = g.size();
            g.gen_range(0, s)
        };
        for _i in 1..size {
            let p = Vector3::new(
                Arbitrary::arbitrary(g),
                Arbitrary::arbitrary(g),
                Arbitrary::arbitrary(g),
            );
            grid.insert_chunk(&p, Arbitrary::arbitrary(g))
        }
        grid
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
        chunk.set_voxel_at(
            Vector3::new(1, 5, 3),
            Material::Grass,
            QuantizedFloat::new(255),
        );
        assert_eq!(
            chunk.get_voxel_at(Vector3::new(1, 5, 3)).get_material(),
            Material::Grass
        );
    }

    #[test]
    fn vg_delete_chunk() {
        let mut vg = VoxelGrid::new();
        let ch = Chunk::new(14);
        vg.insert_chunk(&Vector3::new(0, 0, 0), ch);
        assert!(!vg.is_empty());
        vg.delete_chunk(&Vector3::new(0, 1, 0));
        assert!(!vg.is_empty());
        vg.delete_chunk(&Vector3::new(0, 0, 0));
        assert!(vg.is_empty());
    }

    #[quickcheck]
    fn prop_something() -> bool {
        true
    }
}
