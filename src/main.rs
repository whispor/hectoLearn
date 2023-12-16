mod editor;
use editor::Editor;


fn main() {
    println!("Hello, world!");

    let mut editor = Editor::default();
    editor.run();
}