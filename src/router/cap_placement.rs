#![feature(type_alias_impl_trait)]
use std::collections::HashMap;
pub struct P;

use cap_placement::{calculate_placement, Cap, Net, Via};
use volo_gen::cap_placement::*;

#[volo::async_trait]
impl PlaceService for P {
    async fn get_placements(
        &self,
        req: volo_grpc::Request<PlaceRequest>,
    ) -> core::result::Result<volo_grpc::Response<PlaceResponse>, volo_grpc::Status> {
        let PlaceRequest { vias, nets, caps } = req.get_ref();
        let mut via_map = convert_via(vias.clone());
        let net_map = convert_net(nets.clone());
        let mut cap_map = convert_cap(caps.clone());
        let placements = calculate_placement(&mut via_map, &net_map, &mut cap_map);

        let mut res = vec![];
        for (_, placement) in placements {
            res.push(volo_gen::cap_placement::Placement {
                refdes: placement.refdes.into(),
                symbol_name: placement.symbol_name.into(),
                x_axis: placement.x_axis,
                y_axis: placement.y_axis,
                rotation: placement.rotation.into(),
                mirror: placement.mirror.into(),
            })
        }
        tracing::info!(
            "【Request get_placements】 via_map: {}, net_map: {}, cap_map: {}, res: {}",
            via_map.len(),
            net_map.len(),
            cap_map.len(),
            res.len()
        );
        Ok(volo_grpc::Response::new(PlaceResponse { placements: res }))
    }
}

fn convert_via(vias: Vec<volo_gen::cap_placement::Via>) -> HashMap<i32, Via> {
    let mut via_map: HashMap<i32, Via> = HashMap::new();
    for via in vias {
        via_map.insert(
            via.id,
            Via {
                id: via.id,
                group: via.group.to_string(),
                net: via.net.to_string(),
                circle_drill: via.circle_drill,
                x: via.x,
                y: via.y,
                pair_ids: vec![],
            },
        );
    }
    via_map
}

fn convert_net(nets: Vec<volo_gen::cap_placement::Net>) -> HashMap<String, Net> {
    let mut net_map: HashMap<String, Net> = HashMap::new();
    for n in nets {
        let refdes_list = n.refdes_list.iter().map(|rd| rd.to_string()).collect();
        net_map.insert(
            n.name.to_string(),
            Net {
                name: n.name.to_string(),
                pair_name: n.pair_name.to_string(),
                refdes_list: refdes_list,
            },
        );
    }
    net_map
}

fn convert_cap(caps: Vec<volo_gen::cap_placement::Cap>) -> HashMap<String, Cap> {
    let mut cap_map: HashMap<String, Cap> = HashMap::new();
    for c in caps {
        let mut cap = Cap::new();
        cap.refdes = c.refdes.to_string();
        cap.symbol_name = c.symbol_name.to_string();
        for (_, p) in c.pins {
            cap.insert_pin(p.number, p.net.to_string());
        }
        cap_map.insert(c.refdes.to_string(), cap);
    }
    cap_map
}
