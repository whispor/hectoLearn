mod editor;
use editor::Editor;


fn main() {
    println!("Hello, world!");

    let editor = Editor::default();
    editor.run();
}