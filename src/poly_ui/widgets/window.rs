use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    boxed::Box,
    cell::{Ref, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use super::Owned;
use super::Widget;
use super::WidgetTrait;
use super::WindowProviderTrait;
use super::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Window {
    widget: Owned,
    id: Uuid,
    window_provider: Box<dyn WindowProviderTrait>,
}

//************************************************************************************************
impl Window {
    pub fn new(provider: Box<dyn WindowProviderTrait>) -> Self {
        Self {
            widget: Widget::new().make_ownerless().make_owned(),
            id: Uuid::new_v4(),
            window_provider: provider,
        }
    }
}

//************************************************************************************************
impl WindowTrait for Window {
    fn widget(&self) -> Ref<dyn WidgetTrait> {
        self.widget.get().borrow()
    }

    fn widget_mut(&mut self) -> RefMut<dyn WidgetTrait> {
        self.widget.get().borrow_mut()
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn pos(&self) -> Point2<i32> {
        self.window_provider.pos()
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        self.window_provider.set_pos(new);
    }

    fn size(&self) -> Vector2<u32> {
        self.window_provider.size()
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        self.window_provider.set_size(new);
    }

    fn update(&mut self, dt: f32) {
        self.widget.get().borrow_mut().update(dt);
    }

    fn paint(&mut self) {
        self.window_provider
            .paint_widget(&*self.widget.get().borrow());
    }
}
