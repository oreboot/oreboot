use cursive::{
    align::HAlign,
    views::{Dialog, SelectView, TextView},
    Cursive,
};

fn main() {
    let mut siv = Cursive::default();

    let mut time_select = SelectView::new().h_align(HAlign::Center);
    time_select.add_item("Short", 1);
    time_select.add_item("Medium", 5);
    time_select.add_item("Long", 10);

    time_select.set_on_submit(|s, time| {
        s.pop_layer();
        let text = format!("You will wait for {} minutes...", time);
        s.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
    });

    siv.add_layer(Dialog::around(time_select).title("How long is your wait?"));

    // Starts the event loop.
    siv.run();
}
