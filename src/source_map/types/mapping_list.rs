use super::Mapping;

#[derive(Debug)]
pub struct MappingList {
    pub list: Vec<Mapping>,
    sorted: bool,
    last: Option<Mapping>,
}

impl MappingList {
    pub fn new() -> MappingList {
        MappingList {
            list: Vec::new(),
            sorted: true,
            last: None,
        }
    }

    pub fn add(&mut self, map: Mapping) {
        let is_new_last = if let Some(last_map) = &self.last {
            map >= *last_map
        } else {
            self.last.is_none()
        };

        if is_new_last {
            self.last = Some(map.clone());
            self.list.push(map);
        } else {
            self.sorted = false;
            self.list.push(map);
        }
    }

    pub fn sort(&mut self) {
        if !self.sorted {
            self.list.sort();
            self.sorted = true;
        }
    }
}
