use std::collections::HashMap;

use crate::primitive::Primitive;
use crate::material::Material;

// !todo: remove pub from inner u32
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ObjectId(pub u32);
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MaterialId(pub u32);
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PrimitiveId(pub u32);

#[derive(Copy, Clone)]
struct NextIds {
    object_id: ObjectId,
    material_id: MaterialId,
    primitive_id: PrimitiveId,
}

impl NextIds {
    fn new() -> NextIds {
        NextIds { object_id: ObjectId(0), material_id: MaterialId(0), primitive_id: PrimitiveId(0) }
    }

    fn next_object_id(&mut self) -> ObjectId {
        let id = self.object_id;
        self.object_id.0 += 1;
        id
    }

    fn next_material_id(&mut self) -> MaterialId {
        let id = self.material_id;
        self.material_id.0 += 1;
        id
    }

    fn next_primitive_id(&mut self) -> PrimitiveId {
        let id = self.primitive_id;
        self.primitive_id.0 += 1;
        id
    }
}

pub struct SceneData {
    objects: HashMap<ObjectId, SceneObject>,
    materials: HashMap<MaterialId, Material>,
    primitives: HashMap<PrimitiveId, Primitive>,
    next_ids: NextIds,
}

impl SceneData {
    pub fn new() -> Self {
        SceneData {
            objects: HashMap::new(),
            materials: HashMap::new(),
            primitives: HashMap::new(),
            next_ids: NextIds::new()
        }
    }

    pub fn objects_iter(&self) -> impl ExactSizeIterator<Item=(&ObjectId, &SceneObject)> {
        self.objects.iter()
    }

    pub fn materials_iter(&self) -> impl ExactSizeIterator<Item=(&MaterialId, &Material)> {
        self.materials.iter()
    }

    pub fn primitives_iter(&self) -> impl ExactSizeIterator<Item=(&PrimitiveId, &Primitive)> {
        self.primitives.iter()
    }

    pub fn objects_count(&self) -> usize {
        self.objects.len()
    }

    pub fn materials_count(&self) -> usize {
        self.materials.len()
    }

    pub fn primitives_count(&self) -> usize {
        self.primitives.len()
    }

    pub fn object(&self, id: ObjectId) -> Option<&SceneObject> {
        self.objects.get(&id)
    }

    pub fn material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(&id)
    }

    pub fn primitive(&self, id: PrimitiveId) -> Option<&Primitive> {
        self.primitives.get(&id)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.next_ids.next_material_id();
        self.materials.insert(id, material);
        id
    }

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        let id = self.next_ids.next_primitive_id();
        self.primitives.insert(id, primitive);
        id
    }

    pub fn add_object(&mut self, primitive: PrimitiveId, material: MaterialId) -> Option<ObjectId> {
        if !self.primitives.contains_key(&primitive) || !self.materials.contains_key(&material) {
            return None
        }

        let id = self.next_ids.next_object_id();
        let object = SceneObject::new(primitive, material);
        self.objects.insert(id, object);
        Some(id)
    }

    pub fn create_object(&mut self, primitive: Primitive, material: Material) -> ObjectId {
        let primitive= self.add_primitive(primitive);
        let material= self.add_material(material);

        self.add_object(primitive, material).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct SceneObject {
    primitive: PrimitiveId,
    material: MaterialId,
}

impl SceneObject {
    fn new(primitive: PrimitiveId, material: MaterialId) -> SceneObject {
        SceneObject { primitive, material }
    }

    pub fn primitive(&self) -> PrimitiveId {
        self.primitive
    }

    pub fn material(&self) -> MaterialId {
        self.material
    }
}
