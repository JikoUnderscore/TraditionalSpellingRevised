#![allow(non_snake_case)]
#![windows_subsystem = "windows"]


use fltk::{app, enums, group, text, window};
use fltk::group::Pack;
use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, DisplayExt, WindowExt};
use fltk::text::TextBuffer;
use crate::process::convert;

mod process;
mod static_map;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    const WIN_W: i32 = 1280;
    const WIN_H: i32 = 630;
    let mut root = window::Window::new(100, 100, WIN_W, WIN_H, "Text converter - INPROGRESS BUILD");
    root.size_range(400, 300, i32::MAX, i32::MAX);

    const FONT_SIZE: i32 = 20;
    const ROW: i32 = 50;

    let mut vpack = Pack::new(0, ROW, WIN_W / 2, WIN_H - ROW, "");
    let mut input_textbox = text::TextEditor::new(0, 0, WIN_W / 2, WIN_H, "Enter Text Below:");
    input_textbox.set_label_font(enums::Font::HelveticaBold);
    input_textbox.set_label_size(FONT_SIZE);
    input_textbox.wrap_mode(text::WrapMode::AtBounds, 5);
    input_textbox.set_buffer(Some(text::TextBuffer::default()));
    input_textbox.set_trigger(enums::CallbackTrigger::EnterKeyChanged);

    let mut out_textbox = text::TextDisplay::new(0, 0, WIN_W / 2, WIN_H, "Output Text Below:");
    out_textbox.set_label_font(enums::Font::HelveticaBold);
    out_textbox.set_label_size(FONT_SIZE);
    out_textbox.wrap_mode(text::WrapMode::AtBounds, 5);
    out_textbox.set_buffer(Some(text::TextBuffer::default()));


    vpack.end();
    vpack.set_type(group::PackType::Horizontal);

    root.end();
    root.show();

    let styles = vec![
        text::StyleTableEntry {
            color: enums::Color::from_rgb(0, 0, 0),
            font: enums::Font::HelveticaBold,
            size: 14
        },
        text::StyleTableEntry {
            color: enums::Color::from_rgb(255, 0, 0),
            font: enums::Font::HelveticaBold,
            size: 14
        },
        text::StyleTableEntry {
            color: enums::Color::from_rgb(0, 0, 255),
            font: enums::Font::HelveticaBold,
            size: 14
        },
    ];
    out_textbox.set_highlight_data(out_textbox.buffer(), styles);


    root.handle(move |_, e| {
        if e == enums::Event::Shortcut && app::event_key() == enums::Key::Escape {
            return true;
        }
        return false;
    });

    input_textbox.set_callback({
        out_textbox.clear_changed();
        let mut out_buff = out_textbox.buffer();

        move |e| {
            let text = e.buffer().unwrap().text();
            // println!("{:?}", text);

            let buf = out_buff.as_mut().unwrap();
            let onverter_text = convert(&text.to_lowercase());
            buf.set_text(onverter_text.as_str());
        }
    });

    out_textbox.buffer().unwrap().add_modify_callback({
        let mut curent_output_text = out_textbox.buffer().clone();


        move |pos: i32, n_inserted_chars: i32, n_deleted_chars: i32, n_restyled_chars: i32, deleted_text: &str| {
            if n_inserted_chars == 0 && n_deleted_chars == 0 {
                println!("here");
                return;
            }
            let textbuff = curent_output_text.as_mut().unwrap();

            dbg!(textbuff.text());
            dbg!(textbuff.text_range(textbuff.line_start(pos), textbuff.word_end(pos + n_inserted_chars - n_deleted_chars)));

            println!("___________");

            let new_buff = TextBuffer::default();

            textbuff.highlight(pos, n_inserted_chars / 2);


            dbg!(pos);
            dbg!(n_inserted_chars);
            dbg!(n_deleted_chars);
            dbg!(n_restyled_chars);
            dbg!(deleted_text);
        }
    });

    app.run().unwrap();
}
