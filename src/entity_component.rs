pub trait Component {
    fn update(&self);
    fn typeid(&self) -> i32;
}



pub struct Entity {
    pub comps : Vec<Box<Component>>
}

impl Entity {
    pub fn add_component(& mut self, comp : Box<Component> ) {
        self.comps.push( comp );
    }
    pub fn components(&self) -> &Vec<Box<Component>> {
        &self.comps
    }
}

struct System<'a> {
    comps : Vec<&'a Box<Component>>,
    objects : Vec<Entity>
}

impl<'a> System<'a> {
    pub fn add_entity(& mut self, entity:Entity) {
        
    }
    pub fn update() {
        
    }
}
