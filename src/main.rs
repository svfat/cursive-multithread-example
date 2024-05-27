use cursive::{
    Cursive,
    views::Dialog,
    views::TextView,
    views::ResizedView,
    view::Nameable,
    view::Resizable,
    view::Offset,
    XY,
};
use rand::distributions::{Alphanumeric, Distribution, DistString, Uniform};
use rand::Rng;
use std::thread;
use std::time::Duration;


fn main() {
    let mut siv = cursive::default();
    siv.menubar()
        .add_leaf("𝗦tart", |s| run_new_thread(s))
        .add_leaf("𝗡ext", |s| next_window(s))
        .add_leaf("𝗞ill", |s| kill_thread(s))
        .add_leaf("𝗔bout", |s| s.quit())
        .add_delimiter()
        .add_leaf("𝗤uit", |s| s.quit());
    siv.set_autohide_menu(false);

    siv.add_global_callback('s', |s| run_new_thread(s));
    siv.add_global_callback('n', |s| next_window(s));
    siv.add_global_callback('k', |s| kill_thread(s));
    siv.add_global_callback('a', |s| show_about(s));
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}

fn next_window(siv: &mut Cursive) {
    let screen = siv.screen_mut();
    if screen.len() > 1 {
        screen.move_to_back(cursive::views::LayerPosition::FromFront(0));
    }
}

fn kill_thread(siv: &mut Cursive) {
    todo!()
}

fn show_about(siv: &mut Cursive) {
    todo!()
}

fn run_new_thread(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();
    let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..siv.screen_size().x - 40);
    let y = rng.gen_range(0..siv.screen_size().y - 10);
    let position = XY::new(Offset::Absolute(x), Offset::Absolute(y));

    let text_view = TextView::new(format!("Waiting for data from thread...")).with_name(&name);
    let dialog = Dialog::around(text_view)
        .title(&name)
        .fixed_size((40,10));

    siv.screen_mut().add_layer_at(
        position,
        dialog
    );

    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let char_dist = Uniform::new_inclusive(33u8, 126u8);
        thread::sleep(Duration::from_secs(1)); // short pause before displaying the data
        loop {
            let mut content = String::new();
            for _ in 0..8 {
                let line: String = (0..36)
                    .map(|_| char_dist.sample(&mut rng) as char)
                    .collect();
                content.push_str(&line);
                content.push('\n');
            }
            let name = name.clone();
            thread::sleep(Duration::from_millis(200));
            cb_sink.send(Box::new(move |s| {
                s.call_on_name(&name, |view: &mut TextView| {
                    view.set_content(content);
                });
            })).expect("Failed to send update message to UI")
        }
    });

}

