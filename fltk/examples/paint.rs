use fltk::{app, button::*, draw::*, frame::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = DoubleWindow::new(100, 100, 800, 600, "RustyPainter");
    let mut frame = Frame::new(5, 5, 790, 590, "");
    frame.set_color(Color::White);
    frame.set_frame(FrameType::DownBox);

    wind.end();
    wind.show();

    let offs = Offscreen::new(790, 590).unwrap();
    offs.begin();
    set_draw_color(Color::White);
    draw_rectf(0, 0, 790, 590);
    offs.end();

    let offs = Rc::from(RefCell::from(offs));
    let offs_rc = offs.clone();

    frame.draw(move || {
        if offs_rc.borrow().is_valid() {
            offs_rc.borrow().copy(5, 5, 790, 590, 0, 0);
        }
    });

    let mut x = 0;
    let mut y = 0;

    frame.handle2(move |f, ev| {
        // println!("{:?}", ev);
        // println!("coords {:?}", app::event_coords());
        // println!("get mouse {:?}", app::get_mouse());
        set_draw_color(Color::Red);
        set_line_style(LineStyle::Solid, 3);

        match ev {
            Event::Push => {
                offs.borrow().begin();
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                draw_point(x, y);
                offs.borrow().end();
                f.redraw();
                true
            }
            Event::Drag => {
                offs.borrow().begin();
                let coords = app::event_coords();
                draw_line(x, y, coords.0, coords.1);
                x = coords.0;
                y = coords.1;
                offs.borrow().end();
                f.redraw();
                true
            }
            _ => false,
        }
    });

    app.run().unwrap();
}
