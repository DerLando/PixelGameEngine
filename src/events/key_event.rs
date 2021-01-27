use winit::event::VirtualKeyCode;

pub enum KeyEvent {
    Pressed(VirtualKeyCode),
    Held(VirtualKeyCode),
    Released(VirtualKeyCode)
}