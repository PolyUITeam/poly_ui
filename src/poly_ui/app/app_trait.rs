use super::WindowsManagerTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait AppTrait {
    fn get_windows_manager() -> &mut dyn WindowsManagerTrait;
}