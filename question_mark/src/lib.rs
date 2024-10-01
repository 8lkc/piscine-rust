#[derive(Debug, Clone)]
pub struct Four {pub fourth_layer: Option<u16>}
#[derive(Debug, Clone)]
pub struct Three {pub third_layer: Option<Four>}
#[derive(Debug, Clone)]
pub struct Two {pub second_layer: Option<Three>}
#[derive(Debug, Clone)]
pub struct One {pub first_layer: Option<Two>}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.clone().unwrap()
            .second_layer.clone().unwrap()
            .third_layer.clone().unwrap().fourth_layer
    }
}
