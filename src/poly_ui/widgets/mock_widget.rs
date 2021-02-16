// deps
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
// super
use super::NewWidget;
use super::Ownerless;
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct MockWidget {
    pub update_call_count: u32,
    pub paint_call_count: u32,

    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl MockWidget {
    pub fn new_raw() -> Self {
        Self {
            update_call_count: 0,
            paint_call_count: 0,
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }
}

//************************************************************************************************
impl WidgetTrait for MockWidget {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn add_child(&mut self, child: Ownerless) {
        self.hierarchy.add(child);
    }

    fn remove_child(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
    }

    fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.hierarchy.get_transform(child)
    }

    fn update(&mut self, dt: f32) {
        self.hierarchy.update_children(dt);
        self.update_call_count += 1;
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        self.hierarchy.paint_children(painter);

        let const_self = self as *const Self;
        let mut mut_self = const_self as *mut Self;

        unsafe {
            (*mut_self).paint_call_count += 1;
        }
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn update() {
        let mock = MockWidget::new();

        mock.borrow_mut().update(0.0);
        assert_eq!(mock.borrow().update_call_count, 1);
    }

    //********************************************************************************************
    // #[test]
    // fn paint() {
    //     let mock = MockWidget::new();

    //     mock.borrow_mut().update(0.0);
    //     assert_eq!(mock.borrow().update_call_count, 1);
    // }
}
