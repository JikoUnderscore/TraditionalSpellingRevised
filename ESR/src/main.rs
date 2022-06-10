#![allow(non_snake_case)]
#![windows_subsystem = "windows"]


use std::rc::Rc;
use std::sync::{Arc, Mutex};
use fltk::{app, button, enums, frame, group, input, output, text, window};
use fltk::group::Pack;
use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, DisplayExt, WindowExt, InputExt};
use crate::process::{change_color, convert, web_scpaker};


mod process;
mod static_map;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    const WIN_W: i32 = 1280;
    const WIN_H: i32 = 630;

    let mut root = window::Window::new(100, 100, WIN_W, WIN_H, "Text converter - INPROGRESS BUILD");
    root.size_range(400, 300, i32::MAX, i32::MAX);

    const FONT_SIZE: i32 = 20;
    const HERDER_Y: i32 = 50;
    const ROW: i32 = 50 + HERDER_Y;

    let mut help_button = button::Button::new(WIN_W - 200, 20, 200, 30, "Help add new words!");
    frame::Frame::new(-200, -20, 200, ROW, None)
        .with_align(enums::Align::Right)
        .with_label("Words that are black are confirmed words with no change of spelling.\nWords that are red are unconfirmed words.\nWords that are blue are confirmed words with changed spelling ");

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
    setup_help_window(&mut help_button);

    let styles = vec![
        text::StyleTableEntry {                                // A
            color: enums::Color::from_rgb(0, 0, 0),
            font: enums::Font::HelveticaBold,
            size: 14
        },
        text::StyleTableEntry {                                 // B
            color: enums::Color::from_rgb(255, 0, 0),
            font: enums::Font::HelveticaBold,
            size: 14
        },
        text::StyleTableEntry {                                 // C
            color: enums::Color::from_rgb(0, 0, 255),
            font: enums::Font::HelveticaBold,
            size: 14
        },
    ];

    out_textbox.set_highlight_data(text::TextBuffer::default(), styles);

    root.handle(move |_, e| {
        if e == enums::Event::Shortcut && app::event_key() == enums::Key::Escape {
            return true;
        }
        return false;
    });

    input_textbox.set_callback({
        let mut out_buff = out_textbox.buffer();
        let mut style_buff_ = out_textbox.style_buffer();

        move |input_te| {
            let text = input_te.buffer().unwrap().text().to_lowercase();

            let out_text = out_buff.as_mut().unwrap();
            let converted_input_text = convert(&text.to_lowercase());

            out_text.set_text(converted_input_text.as_str());

            let style_buff_ref = style_buff_.as_mut().unwrap();
            style_buff_ref.set_text("");

            change_color(&text, style_buff_ref);
        }
    });

    app.run().unwrap();
}


fn setup_help_window(help_button: &mut button::Button) {
    const TOP_WINDOW_W: i32 = 900;
    const TOP_WINDOW_H: i32 = 600;

    let mut helo_window = window::DoubleWindow::default()
        .with_size(TOP_WINDOW_W, TOP_WINDOW_H);
    helo_window.make_resizable(true);

    let lable = Arc::new(Mutex::new(frame::Frame::new(0, 200, TOP_WINDOW_W / 2, TOP_WINDOW_H - 200, "")));

    let mut output_window = output::MultilineOutput::new(TOP_WINDOW_W / 2, 200, TOP_WINDOW_W / 2, TOP_WINDOW_H - 200, None);

    const ROW_1: i32 = 20;

    let mut show_webscraped_data = button::ReturnButton::new(550, ROW_1, 100, 50, "Show IPA");
    let mut save_to_output_button = button::Button::new(TOP_WINDOW_W / 2, 150, 100, 50, "Pun in output");

    let mut t = input::Input::new(200, ROW_1, 100, 50, "TS word*");
    t.set_tooltip("Enter the normal spelling of a English word");
    let input_box = Rc::new(t);

    let mut new_spell_box = input::Input::new(400, ROW_1, 100, 50, "TSR word*");
    new_spell_box.set_tooltip("Enter the new spelling of the word.");

    helo_window.end();

    save_to_output_button.set_callback({
        let input = input_box.clone();

        move |_| {
            let curent_text = output_window.value();

            output_window.set_value(&format!("{}{} {}\n", curent_text, input.value(), new_spell_box.value()));
        }
    });


    show_webscraped_data.set_callback({
        let input_ref = input_box.clone();

        move |_| {
            let lable_ref = lable.clone();
            let input = input_ref.as_ref().value();

            {
                let mut lock = lable_ref.lock().unwrap();
                lock.set_label("Searching...");
            }

            std::thread::spawn(move || {
                match web_scpaker(&input) {
                    Ok(vec_ipa_str) => {
                        let mut lock = lable_ref.lock().unwrap();
                        if vec_ipa_str.is_empty() {
                            lock.set_label("Error. Word not found!");
                        } else {
                            lock.set_label(&vec_ipa_str);
                        }
                    }
                    Err(e) => {
                        let mut lock = lable_ref.lock().unwrap();
                        lock.set_label(&e.to_string());
                    }
                };
            });
        }
    });


    help_button.set_callback(move |_| {
        helo_window.show();
    });
}

