use std::mem::transmute;

use libxdsim::{app_state::*, component::*, graphics::*};
use xdsim_dummies_type::Bit;

#[repr(C)]
pub struct NoProperties;

impl PropertiesContainer for NoProperties {
    fn get_menu(&self) -> Menu {
        Menu { items: vec![] }
    }

    fn get_option(&self) -> Option<MenuInputValue> {
        None
    }

    fn set_option(
        &mut self,
        _id: &str,
        _value: MenuInputValue,
    ) -> Result<(), PropertiesContainerSetError> {
        Err(PropertiesContainerSetError::PropertyDoesNotExist)
    }

    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

#[repr(C)]
pub struct TFlipFlop {
    value: Bit,
    properties: NoProperties,
}

impl Gate for TFlipFlop {
    fn definition(&self) -> GateDefinition {
        GateDefinition {
            version: 0,
            inputs: vec![GateIOEntry {
                name: "T".to_string(),
                data_type: "dummies-0.1::bit",
                position: Vec2 { x: 0.0, y: 0.5 },
            }],
            outputs: vec![
                GateIOEntry {
                    name: "Q".to_string(),
                    data_type: "dummies-0.1::bit",
                    position: Vec2 { x: 1.0, y: 0.7 },
                },
                GateIOEntry {
                    name: "QBar".to_string(),
                    data_type: "dummies-0.1::bit",
                    position: Vec2 { x: 1.0, y: 0.3 },
                },
            ],
            bounding_box: Vec2 { x: 1.0, y: 1.0 },
            identifier: "dummy-t-flip-flop-0.1",
        }
    }

    fn tick(&mut self, input: GateTickRequest) -> Vec<*const ()> {
        let bit = &unsafe { *(input.inputs[0] as *const Bit) };

        if bit.0 {
            self.value.0 = !self.value.0;
        }

        // return (self.value,);
        unsafe { vec![transmute(Box::into_raw(Box::new(self.value)))] }
    }

    fn draw(&self, _request: &GateDrawRequest) -> libxdsim::graphics::Graphic {
        Graphic::from_vec(vec![
            Element::Rect {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size: Vec2 { x: 0.0, y: 0.0 },
                stroke: StrokeStyle { colour: Colour::Fg },
                fill: FillStyle {
                    colour: Colour::Transparent,
                },
            },
            Element::Rect {
                pos: Vec2 { x: 0.4, y: 0.4 },
                size: Vec2 { x: 0.2, y: 0.2 },
                stroke: StrokeStyle {
                    colour: Colour::Black,
                },
                fill: FillStyle {
                    colour: if self.value.0 {
                        Colour::Black
                    } else {
                        Colour::Yellow
                    },
                },
            },
        ])
    }

    fn properties_container(&self) -> &dyn PropertiesContainer {
        &self.properties
    }

    fn properties_container_mut(&mut self) -> &mut dyn PropertiesContainer {
        &mut self.properties
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.serialize()
    }
}

pub fn create_gate() -> Box<dyn Gate> {
    Box::new(TFlipFlop {
        value: Bit(false),
        properties: NoProperties,
    })
}

pub fn deserialize_gate(gate: Vec<u8>, properties: Vec<u8>) -> Box<dyn Gate> {
    Box::new(TFlipFlop {
        value: Bit::deserialize(gate),
        properties: deserialize_gate_property(properties),
    })
}

pub fn deserialize_gate_property(_properties: Vec<u8>) -> NoProperties {
    NoProperties
}
