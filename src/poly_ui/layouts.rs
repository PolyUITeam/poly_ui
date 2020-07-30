use nalgebra::Vector3;
use uuid::Uuid;
use std::{collections::HashMap, fmt::Debug, rc::Rc, cell::RefCell};

use crate::poly_ui::widgets::Widget;
use crate::poly_ui::components::Hierarchy;

// traits
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait Layout: Debug {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>);    
    
    fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>);
}

// impl
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct CanvasLayout {
    children: HashMap<Uuid, Vector3<i32>>,
    hierarchy: Rc<RefCell<Hierarchy>>,
}

impl CanvasLayout {
    pub fn new() -> Self {
        return Self {
            children: HashMap::new(),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
        };
    }
}

impl Layout for CanvasLayout {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>) {
        self.hierarchy = hierarchy;
    }

    fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>) {
        self.children.insert(*child.borrow().id(), pos);
        self.hierarchy.borrow_mut().add(child);
    }
}

// tests
//********************************************************************************************
//********************************************************************************************
//********************************************************************************************
#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use std::{rc::Rc, cell::RefCell};

    use crate::poly_ui::widgets::{Widget, BaseWidget};
    use crate::poly_ui::layouts::CanvasLayout;
    
    //****************************************************************************************
    #[test]
    fn canvas_layout_add_child() {
        let mut parent_widget = BaseWidget::new();
        parent_widget.set_layout(Box::new(CanvasLayout::new()));
        let child_widget = Rc::new(RefCell::new(BaseWidget::new()));
        parent_widget.layout_mut().add(child_widget.clone(), Vector3::<i32>::new(1, 2, 0));
        
        assert_eq!(parent_widget.hierarchy().children()[0].borrow().id(), child_widget.borrow().id());
    }
}