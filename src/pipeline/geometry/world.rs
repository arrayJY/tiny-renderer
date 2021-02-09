use super::model::Model;
use crate::algebra::matrix::Matrix4f;
use std::collections::HashMap;

pub struct ModelInWorld {
    pub model: Model,
    pub transform_matrix: Matrix4f,
}

#[allow(dead_code)]
impl ModelInWorld {
    fn new(model: Model) -> ModelInWorld {
        ModelInWorld {
            model,
            transform_matrix: Matrix4f::unit(),
        }
    }

    fn model(&self) -> &Model {
        &self.model
    }
    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    fn transform_matrix(&self) -> &Matrix4f {
        &self.transform_matrix
    }

    fn transform_matrix_mut(&mut self) -> &mut Matrix4f {
        &mut self.transform_matrix
    }

    // generate the model after tranformating.
    // it spend a lot of time.
    fn model_in_world(&self) -> Model {
        Model {
            indices: self.model().indices().clone(),
            vertexs: self
                .model()
                .vertexs()
                .iter()
                .map(|vertex| self.transform_matrix() * vertex)
                .collect(),
        }
    }
}
pub struct World {
    models: HashMap<String, ModelInWorld>,
}

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        World {
            models: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Model> {
        match self.models.get(&String::from(name)) {
            Some(model_in_world) => Some(model_in_world.model()),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Model> {
        match self.models.get_mut(&String::from(name)) {
            Some(model_in_world) => Some(model_in_world.model_mut()),
            _ => None,
        }
    }

    pub fn insert(&mut self, name: &str, model: Model) {
        self.models
            .insert(String::from(name), ModelInWorld::new(model));
    }

    pub fn remove(&mut self, name: &str) -> Option<Model> {
        match self.models.remove(&String::from(name)) {
            Some(model_in_world) => Some(model_in_world.model),
            _ => None,
        }
    }

    //Get one model.
    pub fn model_in_world(&self, name: &str) -> Option<Model> {
        match self.models.get(&String::from(name)) {
            Some(model) => Some(model.model_in_world()),
            _ => None,
        }
    }

    //Get all models.
    pub fn models_in_world(&self) -> HashMap<String, Model> {
        self.models
            .iter()
            .map(|(name, model)| (String::from(name), model.model_in_world()))
            .collect()
    }

    pub fn translate(&mut self, name: &str, (x, y, z): (f32, f32, f32)) {
        self.transform(name, &Matrix4f::translation_matrix(x, y, z))
    }

    pub fn scale(&mut self, name: &str, (sx, sy, sz): (f32, f32, f32)) {
        self.transform(name, &Matrix4f::scale_matrix(sx, sy, sz))
    }

    pub fn rotate_around_x(&mut self, name: &str, angle: f32) {
        self.rotate_around_axis(name, angle, "x")
    }

    pub fn rotate_around_y(&mut self, name: &str, angle: f32) {
        self.rotate_around_axis(name, angle, "y")
    }

    pub fn rotate_around_z(&mut self, name: &str, angle: f32) {
        self.rotate_around_axis(name, angle, "z")
    }

    fn rotate_around_axis(&mut self, name: &str, angle: f32, axis: &str) {
        let rotate_matrix = match axis {
            "x" => Matrix4f::rotate_around_x_matrix(angle),
            "y" => Matrix4f::rotate_around_y_matrix(angle),
            "z" => Matrix4f::rotate_around_z_matrix(angle),
            _ => panic!("Rotate around unexpected axis."),
        };
        self.transform(name, &rotate_matrix)
    }

    fn transform(&mut self, name: &str, matrix: &Matrix4f) {
        if let Some(model_in_world) = self.models.get_mut(&String::from(name)) {
            model_in_world.transform_matrix = &model_in_world.transform_matrix * matrix;
        }
    }
}