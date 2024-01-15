/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

mod xcli;
use xcli::cli;

#[tokio::main]
async fn main() {
    cli::cli_main().await;
}
/*
 use cursive::theme::{Color, PaletteColor};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    let mut theme = cursive::theme::load_default();
    theme.palette[PaletteColor::Background] = Color::Rgb(85, 85, 85);

    // Modify the background color of the default input color to black
    theme.palette[PaletteColor::Secondary] = Color::Rgb(0, 0, 0);

    siv.set_theme(theme);

    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    siv.add_layer(
        Dialog::new()
            .title("Enter your name")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(show_popup)
                    .with_name("name")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let name = s
                    .call_on_name("name", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();

                // Run the next step
                show_popup(s, &name);
            }),
    );

    siv.run();
}

// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        // Try again as many times as we need!
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {name}!");
        // Remove the initial popup
        s.pop_layer();
        // And put a new one instead
        s.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
    }
}
  */
