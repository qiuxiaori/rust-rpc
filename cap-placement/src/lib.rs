use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Via {
    pub id: i32,
    pub pair_ids: Vec<i32>,
    pub group: String,
    pub net: String,
    pub circle_drill: f32,
    pub x: f32,
    pub y: f32,
}

impl Via {
    pub fn new() -> Self {
        Via {
            id: 0,
            pair_ids: Vec::new(),
            group: String::new(),
            net: String::new(),
            circle_drill: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn add_pair_ids(&mut self, pair_id: i32) {
        self.pair_ids.push(pair_id);
    }

    pub fn set_net(&mut self, net: String) {
        self.net = net;
    }
}

#[derive(Clone, Debug)]
pub struct Placement {
    pub refdes: String,
    pub symbol_name: String,
    pub x_axis: f32,
    pub y_axis: f32,
    pub rotation: String,
    pub mirror: String,
    pub is_reverse: bool,
}

#[derive(Clone, Debug)]
pub struct Cap {
    pub refdes: String,
    pub symbol_name: String,
    pub pins: HashMap<String, Pin>,
}

#[derive(Clone, Debug)]
pub struct Pin {
    pub number: i32,
    pub net: String,
    pub via_id: i32,
}

impl Cap {
    pub fn new() -> Self {
        Cap {
            refdes: String::new(),
            symbol_name: String::new(),
            pins: HashMap::new(),
        }
    }

    pub fn insert_pin(&mut self, pin_number: i32, net: String) {
        let pin = Pin {
            number: pin_number,
            net: net.clone(),
            via_id: 0,
        };
        if net.contains("GND") {
            self.pins.insert("gnd".to_string(), pin);
        } else {
            self.pins.insert("power".to_string(), pin);
        }
    }

    pub fn set_power_gnd_id(&mut self, power_id: i32, gnd_id: i32) {
        let power_pin = self.pins.get_mut("power").unwrap();
        power_pin.via_id = power_id;

        let gnd_pin = self.pins.get_mut("gnd").unwrap();
        gnd_pin.via_id = gnd_id;
    }

    pub fn is_place(&self) -> bool {
        let power_pin = self.pins.get("power").unwrap();
        let gnd_pin = self.pins.get("gnd").unwrap();
        power_pin.via_id != 0 && gnd_pin.via_id != 0
    }
}

#[derive(Clone, Debug)]
pub struct Net {
    pub name: String,
    pub pair_name: String,
    pub refdes_list: HashSet<String>, // type: String, // 预留， 暂时只存储 power net
}

impl Net {
    pub fn new(name: String) -> Self {
        Net {
            name,
            pair_name: String::new(),
            refdes_list: HashSet::new(),
        }
    }

    pub fn set_pair_name(&mut self, pair_name: String) {
        self.pair_name = pair_name;
    }

    pub fn add_cap(&mut self, refdes: String) {
        self.refdes_list.insert(refdes);
    }
}

pub struct CapStandard {
    pub _gap: f32,
    pub length: f32,
    pub width: f32,
}

pub fn calculate_placement(
    via_map: &mut HashMap<i32, Via>,
    net_map: &HashMap<String, Net>,
    cap_map: &mut HashMap<String, Cap>,
) -> HashMap<String, Placement> {
    let mut placement_map = HashMap::new();
    placement_map.insert("aaa".to_string(), Placement {
        refdes: "refdes".to_string(),
        symbol_name: "symbol_name".to_string(),
        x_axis: 33.0,
        y_axis: 22.0,
        rotation: "0".to_string(),
        mirror: "m".to_string(),
        is_reverse: false
    });
    placement_map
}
