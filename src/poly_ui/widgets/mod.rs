mod widget_trait;
mod window_trait;
mod window_provider_trait;
mod widget;
mod window;

pub use widget_trait::WidgetTrait;
pub use widget_trait::update_children;
pub use widget_trait::paint_children;
pub use window_trait::WindowTrait;
pub use window_provider_trait::WindowProviderTrait;
pub use widget::Widget;
pub use window::Window;