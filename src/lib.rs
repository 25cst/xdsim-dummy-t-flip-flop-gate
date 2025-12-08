use libxdsim::{app_state::*, component::*, graphics::*};
use xdsim_dummy_bit_type::Bit;

#[repr(C)]
pub struct NoProperties;

impl PropertiesContainer for NoProperties {
    fn get_menu(&self) -> Menu {
        Menu { items: Box::new([]) }
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

    fn serialize(&self) -> Box<[u8]> {
        Box::new([])
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
            inputs: Box::new([GateIOEntry {
                name: "T".into(),
                data_type: ("dummy-bit", 0, 1),
                position: Vec2 { x: 0.0, y: 0.5 },
            }]),
            outputs: Box::new([
                GateIOEntry {
                    name: "Q".into(),
                    data_type: ("dummy-bit", 0, 1),
                    position: Vec2 { x: 1.0, y: 0.7 },
                },
                GateIOEntry {
                    name: "QBar".into(),
                    data_type: ("dummy-bit", 0, 1),
                    position: Vec2 { x: 1.0, y: 0.3 },
                },
            ]),
            bounding_box: Vec2 { x: 1.0, y: 1.0 },
            identifier: ("dummy-t-flip-flop", 0, 1),
        }
    }

    fn tick(&mut self, input: GateTickRequest) -> Box<[Box<dyn Type>]> {
        let bit = input.get_input::<Bit>(0);

        if bit.0 {
            self.value.0 = !self.value.0;
        }

        Box::new([Box::new(self.value)])
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

    fn serialize(&self) -> Box<[u8]> {
        self.value.serialize()
    }
}

pub fn create_gate() -> Box<dyn Gate> {
    Box::new(TFlipFlop {
        value: Bit(false),
        properties: NoProperties,
    })
}

pub fn deserialize_gate(gate: Box<[u8]>, _props: Box<[u8]>) -> Box<dyn Gate> {
    Box::new(TFlipFlop {
        value: xdsim_dummy_bit_type::deserialize(gate),
        properties: NoProperties,
    })
}
