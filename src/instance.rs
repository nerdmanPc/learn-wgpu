use cgmath::{
    Matrix4,
    Vector3, 
    Quaternion,
    Zero,
    Rotation3,
    InnerSpace, 
    Deg,
};
use wgpu::util::DeviceExt;

const NUM_INSTANCES_PER_ROW: u32 = 10;
const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW ^ 2;
const INSTANCE_DISPLACEMENT: Vector3<f32> = Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

pub struct State {

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,

} impl State {

    pub fn new(device: &wgpu::Device) -> Self {
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map( |z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = 
                Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;
                let rotation = if position.is_zero() {
                    Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0))
                } else {
                    Quaternion::from_axis_angle(position.normalize(), Deg(45.0))
                };

                Instance { position, rotation }
            })
        }).collect::<Vec<Instance>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<InstanceRaw>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        Self { instances, instance_buffer }
    }

    pub fn buffer<'s>(&'s self) -> &'s wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn num_instances(&self) -> usize {
        self.instances.len()
    }
}

struct Instance {

    position: Vector3<f32>,
    rotation: Quaternion<f32>,

} impl Instance {

    fn to_raw(&self) -> InstanceRaw {
        let translation_matrix = Matrix4::from_translation(self.position);
        let rotation_matrix = Matrix4::from(self.rotation);
        let model_matrix = (translation_matrix * rotation_matrix).into();

        InstanceRaw { model_matrix }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {

    model_matrix: [[f32; 4]; 4],

} impl InstanceRaw {

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}