use vecmath::Matrix4;
use crate::gfx::ui::{Callbacks, ui_counter, UIElement, UIElementData, UIRenderData};
use crate::{BufferDataType, BufferType, Constructor, CustomUIProperty, DrawType, LocalAttribPointer, MatrixWrapper, UI, VertexArrayObject, VertexArrayObjectType, VertexBufferObject};
use crate::math::linear_algebra::IDENTITY_MAT4;

pub const BUFFER: [f32; 16] = [
    0f32, 1f32, 0f32, 1f32, //A
    0f32, 0f32, 0f32, 0f32, //B
    1f32, 1f32, 1f32, 1f32,//C
    1f32, 0f32, 1f32, 0f32//D
];

#[derive(Debug)]
pub struct Rectangle {
    data: UIElementData,
    size: [f32; 2],
}

impl Rectangle {
    pub fn with_position(mut self, position: [u32; 2]) -> Self {
        self.set_position(position);
        self
    }

    pub fn with_custom_property(mut self, property: &dyn ToString, value: CustomUIProperty) -> Self {
        self.set_custom_property(property, value);
        self
    }

    pub fn scale(&self) -> Matrix4<f32> {
        MatrixWrapper(IDENTITY_MAT4).scale([self.size[0], self.size[1], 0f32]).0
    }
    pub fn new(position: [u32; 2], size: [f32; 2]) -> Self {
        let buffer = BUFFER;

        let vao = VertexArrayObject::new(Some(VertexArrayObjectType::ArrayStrips(4)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, DrawType::StaticDraw, &buffer))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .build();
        Self {
            data: UIElementData {
                id: ui_counter(),
                parent: None,
                children: vec![],
                position,
                width: size[0] as u32,
                height: size[1] as u32,
                tmp_children: vec![],
                render_data: Some(UIRenderData(UI::default_program(), vao, Default::default())),
                hover_flag: false,
                custom_properties: Default::default(),
                corner_radius: 0.0,
                drag_offset: None,
                callbacks: Callbacks { on_drag: vec![] },
            },
            size,
        }
    }
    pub fn data(&self) -> &UIElementData {
        &self.data
    }
    pub fn size(&self) -> [f32; 2] {
        self.size
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new([0; 2], [100f32, 30f32])
    }
}

impl UIElement for Rectangle {
    fn tag(&self) -> &'static str {
        "Rectangle"
    }

    fn element_data(&self) -> &UIElementData {
        &self.data
    }

    fn element_data_mut(&mut self) -> &mut UIElementData {
        &mut self.data
    }

    fn resize(&mut self, _size: [i32; 2]) {}

    fn contains_point(&self, point: [f64; 2]) -> bool {
        let x = self.position()[0] as f32;
        let y = self.position()[1] as f32;
        let w = self.size[0] + x;
        let h = self.size[1] + y;
        point[0] as f32 >= x && point[0] as f32 <= w
            &&
            point[1] as f32 >= y && point[1] as f32 <= h
    }
}