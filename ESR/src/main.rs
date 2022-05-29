#![allow(non_snake_case)]
#![windows_subsystem = "windows"]


use fltk::{app, enums, group, text, window};
use fltk::group::Pack;
use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, DisplayExt};
use crate::process::convert;

mod process;
mod static_map;

fn main(){


    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    const WIN_W: i32 = 1280;
    const WIN_H: i32 = 630;
    let mut root = window::Window::new(100, 100, WIN_W, WIN_H, "Text converter - INPROGRESS BUILD");
    // root.size_range(400, 300, i32::MAX, i32::MAX);

    const FONT_SIZE: i32 = 20;
    const ROW: i32 = 50;

    // let mut enter_button = button::Button::new((WIN_W / 2) - 50, 10, 120, 50, "Shift + Enter!");
    // enter_button.set_shortcut(enums::Shortcut::Shift | enums::Key::Enter);

    let mut vpack = Pack::new(0, ROW, WIN_W / 2, WIN_H - ROW, "");
    let mut input_textbox = text::TextEditor::new(0, 0, WIN_W /2, WIN_H , "Enter Text Below:");
    input_textbox.set_label_font(enums::Font::HelveticaBold);
    input_textbox.set_label_size(FONT_SIZE);
    input_textbox.wrap_mode(text::WrapMode::AtBounds, 5);
    input_textbox.set_buffer(Some(text::TextBuffer::default()));
    input_textbox.set_trigger(enums::CallbackTrigger::EnterKeyChanged);

    let mut out_textbox = text::TextDisplay::new(0, 0, WIN_W /2, WIN_H , "Output Text Below:");
    out_textbox.set_label_font(enums::Font::HelveticaBold);
    out_textbox.set_label_size(FONT_SIZE);
    out_textbox.wrap_mode(text::WrapMode::AtBounds, 5);
    out_textbox.set_buffer(Some(text::TextBuffer::default()));

    vpack.end();
    vpack.set_type(group::PackType::Horizontal);



    root.end();
    root.show();

    root.handle(move |_, e| {
        if e == enums::Event::Shortcut && app::event_key() == enums::Key::Escape {
            return true;
        }
        return false;
    });

    input_textbox.set_callback({
        out_textbox.clear_changed();
        let mut out_buff = out_textbox.buffer();

        move |e |{
            let text = e.buffer().unwrap().text();
            // println!("{:?}", text);

            let buf = out_buff.as_mut().unwrap();

            buf.set_text(convert(&text.to_lowercase()).as_str());
        }
    });


    // enter_button.set_callback({
    //     let buf = input_textbox.buffer();
    //     let mut out_buff = out_textbox.buffer();
    //
    //     move |e| {
    //         let text = buf.as_ref().unwrap().text();
    //         println!("{:?}", text);
    //         out_buff.as_mut().unwrap().set_text(text.as_str());
    //     }
    // });

    app.run().unwrap();
}
