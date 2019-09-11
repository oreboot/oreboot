use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

fn main() {
    let mut siv = Cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
}
