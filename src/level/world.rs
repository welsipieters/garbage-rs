use rand::prelude::*;
use crate::constants::{ENTITY_LENGTH, MAX_WORLD_ENTITIES};
use rand::distributions::{Distribution, Uniform};

pub struct World {
    // TODO: Idk this feels wrong.
    world_data: Vec<u16>,
    world_data_new: Vec<u16>,
    assigned: usize,
}

impl World {
    pub fn new() -> World {
        World {
            assigned: 0,
            world_data: vec![0_u16; MAX_WORLD_ENTITIES * ENTITY_LENGTH],
            world_data_new: vec![0_u16; MAX_WORLD_ENTITIES * ENTITY_LENGTH],
        }
    }

    pub fn write(&mut self, start_index: &usize, bytes: Vec<u16>) {
        for (index, data) in bytes.into_iter().enumerate() {
            self.world_data_new[start_index + index] = data;
        }
    }

    pub fn diff(&mut self) -> DiffReport {
        let mut contiguous_sections: Vec<(usize, usize)> = Vec::new();
        let mut changed_indices: Vec<usize> = Vec::new();
        let mut changed_bytes: Vec<Vec<u16>> = Vec::new();

        for index in 0..self.world_data.len() {
            if self.world_data[index] != self.world_data_new[index] {
                changed_indices.push(index)
            }

            self.world_data[index] = self.world_data_new[index];
        }

        let mut start_of_section = 0;
        for i in 0..changed_indices.len() {
            if i + 1 != changed_indices[i + i] {
                contiguous_sections.push((start_of_section, i));
                start_of_section = i + 1;
            }
        }

        contiguous_sections.iter().for_each(|(start, end)| {
            let index_start = changed_indices[start.clone()];
            let index_end = changed_indices[end.clone()];
            let mut bytes = self.world_data[index_start..index_end].to_owned();

            changed_bytes.push(bytes);
        });

        return DiffReport {
            contiguous_sections,
            changed_indices,
            changed_bytes,
        };
    }

    // I laugh every time I see that Bill named this malloc, lol
    pub fn malloc(&mut self) -> (u16, u16) {
        let ids = self
            .world_data_new
            .iter()
            .enumerate()
            .filter(|(id, _)| id % 8 == 0)
            .map(|(_, val)| val.clone())
            .collect::<Vec<_>>();

        let mut address: u16 = 0;

        for i in (0..self.world_data_new.len()).step_by(ENTITY_LENGTH) {
            if self.world_data_new[i] == 0_u16 {
                address = i as u16;
                break;
            }
        }

        let mut id: u16 = 0;
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(0..0_u16);

        while ids.contains(&id) {
            id = uniform.sample(&mut rng);
        }

        self.world_data_new[address as usize] = id;

        (address, id)
    }
}

pub struct DiffReport {
    contiguous_sections: Vec<(usize, usize)>,
    changed_indices: Vec<usize>,
    changed_bytes: Vec<Vec<u16>>,
}

impl DiffReport {
    pub fn as_packets(&self) -> Vec<Packet> {
        let mut packets = vec![];

        self.contiguous_sections
            .iter()
            .enumerate()
            .for_each(|(id, section)| {
                let bytes = &self.changed_bytes[id];

                packets.push(Packet {
                    address: self.changed_indices[section.0],
                    bytes: bytes.clone(),
                    length: bytes.len(),
                })
            });

        packets
    }
}

pub struct Packet {
    address: usize,
    length: usize,
    bytes: Vec<u16>,
}
