
pub fn run() -> Result<(), slint::PlatformError> {
    MainWindow::new().unwrap().run()
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "Hello, World!";
            color: green;
        }
    }
}