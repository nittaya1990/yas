use enigo::{Enigo, MouseControllable, MouseButton};

pub struct WindowsSystemControl {
    enigo: Enigo,
}

impl WindowsSystemControl {
    pub fn new() -> WindowsSystemControl {
        WindowsSystemControl { enigo: Enigo::new() }
    }

    pub fn mouse_move_to(&mut self, x: i32, y: i32) -> anyhow::Result<()> {
        self.enigo.mouse_move_to(x, y);

        anyhow::Ok(())
    }

    pub fn mouse_click(&mut self) -> anyhow::Result<()> {
        self.enigo.mouse_click(MouseButton::Left);

        anyhow::Ok(())
    }

    pub fn mouse_scroll(&mut self, amount: i32, _try_find: bool) -> anyhow::Result<()> {
        self.enigo.mouse_scroll_y(amount);

        anyhow::Ok(())
    }
}
