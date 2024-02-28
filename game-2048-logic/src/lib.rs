slint::include_modules!();

use slint::PlatformError;

#[inline]
pub fn game_main_entry() -> Result<(), PlatformError> {
    MainWindow::new()?.run()
}