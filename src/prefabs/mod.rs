use crate::constants::{AGREED_UPON_BITS,  ENTITY_LENGTH};
use hashbrown::HashMap;

pub struct Prefab {
    template: Template,
    attrs: HashMap<String, u16>,
    entity_id: u16,
    entity_addr: u16,
    dirty: bool,
}

impl Prefab {
    pub fn new() -> Prefab {
        Prefab {
            template: Template::new(),
            attrs: HashMap::new(),
            entity_id: 0,
            entity_addr: 0,
            dirty: false,
        }
    }

    pub fn unpack(&mut self) {
        self.attrs = self.template.unpack();
    }

    pub fn write_attr(&mut self, key: String, value: u16) {
        self.touch();

        self.attrs.insert(key, value);
        self.repack();
    }

    pub fn repack(&mut self) {
        self.template.pack(&self.attrs);
    }

    pub fn touch(&mut self) {
        self.dirty = true;
    }

    pub fn set_entity_id(&mut self, id: u16) {
        self.entity_id = id;
    }

    pub fn set_entity_addr(&mut self, addr: u16) {
        self.entity_addr = addr;
    }
}

pub struct Template {
    scratch_pad: Vec<u16>,
    properties: HashMap<String, usize>,
}

impl Template {
    pub fn new() -> Template {
        Template {
            scratch_pad: vec![0_u16; ENTITY_LENGTH * 2],
            properties: HashMap::new(),
        }
    }

    pub fn load(&mut self, properties: HashMap<String, usize>) {
        self.properties.extend(properties)
    }

    pub fn add_property(&mut self, key: String, bits: usize) {
        self.properties.insert(key, bits);
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn pack_raw_bytes(&mut self, bytes: Vec<u16>) {
        bytes.into_iter().enumerate().for_each(|(index, byte)| {
            self.scratch_pad[index] = byte;
        });
    }

    pub fn pack(&mut self, values: &HashMap<String, u16>) {
        let mut running_bits = 0;
        let mut running_uint16s = 0;
        let mut pack_value = 0;
        let mut distance_to_edge = 0;

        for (key, bits) in self.properties.iter() {
            distance_to_edge =
                (AGREED_UPON_BITS - ((running_bits + bits) % AGREED_UPON_BITS)) % AGREED_UPON_BITS;
            pack_value |= (values.get(key).unwrap_or(&0) << distance_to_edge);
            running_bits += bits;

            self.scratch_pad[running_uint16s] = pack_value;

            if distance_to_edge == 0 {
                running_uint16s += 1;
                pack_value = 0;
            }
        }
    }

    pub fn unpack(&self) -> HashMap<String, u16> {
        let mut values = HashMap::new();
        let mut bounds = (0, 0);

        for (key, bits) in self.properties.iter() {
            bounds.1 = bounds.0 + bits;

            values.insert(key.into(), self.unpack_from_buffer(bounds));
            bounds.0 = bounds.1
        }

        values
    }

    fn unpack_from_buffer(&self, bounds: (usize, usize)) -> u16 {
        let bit_masks: Vec<u16> = vec![
            0b0,
            0b1,
            0b11,
            0b111,
            0b1111,
            0b11111,
            0b111111,
            0b1111111,
            0b11111111,
            0b111111111,
            0b1111111111,
            0b11111111111,
            0b111111111111,
            0b1111111111111,
            0b11111111111111,
            0b111111111111111,
            0b1111111111111111,
        ];

        let bit_length = bounds.1 - bounds.0;

        let range_start = (bounds.0 / AGREED_UPON_BITS);
        let range_end = (bounds.1 / AGREED_UPON_BITS);

        let distance_to_edge =
            (AGREED_UPON_BITS - (bounds.1 % AGREED_UPON_BITS)) % AGREED_UPON_BITS;
        let mask = bit_masks[bit_length] << distance_to_edge;

        (self.scratch_pad[range_start] & mask) >> distance_to_edge
    }
}
