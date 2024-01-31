use crate::key_manager::key_types::EvdevKeyCode;

pub struct KeySize(pub usize);

pub struct PhysicalRow {
    pub keys: Vec<(EvdevKeyCode, KeySize)>,
}

pub struct PhysicalLayout {
    pub rows: Vec<PhysicalRow>,
}

impl From<Vec<Vec<usize>>> for PhysicalLayout {
    fn from(val: Vec<Vec<usize>>) -> PhysicalLayout {

        PhysicalLayout {
            rows: vec![PhysicalRow {
                keys: vec![(EvdevKeyCode(1), KeySize(2))],
            }],
        }
    }
}
