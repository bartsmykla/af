use io::BufWriter;
use powerpack::{Icon, Item, Key, Kind, Modifier, Output, value};
use std::{env, error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1);

    // Create an item to show in the Alfred drop down.
    let kuma = Item::new("kumahq/kuma")
        .subtitle(format!("Your query was '{query:?}'"))
        .uid("kumahq/kuma")
        .arg("kumahq/kuma")
        .icon(Icon::with_image("github.svg"))
        .valid(true)
        .matches("use this to filter")
        .autocomplete("to this")
        .kind(Kind::FileSkipCheck)
        .copy_text("https://github.com/kumahq/kuma")
        .large_type_text("https://github.com/kumahq/kuma")
        .modifier(Modifier::new(Key::Command).subtitle("⌘ changes the subtitle"))
        .modifier(Modifier::new(Key::Option).arg("/path/to/modified.jpg"))
        .modifier(
            Modifier::new_multi([Key::Control, Key::Shift]).icon(Icon::with_image(
                "/Users/bart.smykla@konghq.com/Downloads/af-avatar.png",
            )),
        )
        .modifier(Modifier::new(Key::Shift).valid(false))
        .quicklook_url("https://github.com/kumahq/kuma")
        .action(value!({
            "text": ["one", "two", "three"],
            "url": "https://www.alfredapp.com",
            "file": "~/Desktop",
            "auto": "~/Pictures"
        }));

    // Create an item to show in the Alfred drop down.
    let kong_mesh = Item::new("Kong/kong-mesh")
        .subtitle(format!("Your query was '{query:?}'"))
        .uid("Kong/kong-mesh")
        .arg("https://github.com/Kong/kong-mesh")
        .icon(Icon::with_image("github.svg"))
        // .valid(true)
        // .matches("use this to filter")
        // .autocomplete("to this")
        // .kind(Kind::Default)
        .copy_text("https://github.com/Kong/kong-mesh")
        .large_type_text("https://github.com/Kong/kong-mesh")
        // .modifier(Modifier::new(Key::Command).subtitle("⌘ changes the subtitle"))
        // .modifier(Modifier::new(Key::Option).arg("/path/to/modified.jpg"))
        // .modifier(
        //     Modifier::new_multi([Key::Control, Key::Shift]).icon(Icon::with_image(
        //         "/Users/bart.smykla@konghq.com/Downloads/af-avatar.png",
        //     )),
        // )
        // .modifier(Modifier::new(Key::Shift).valid(false))
        .quicklook_url("https://github.com/Kong/kong-mesh")
        .action(value!({
            "text": ["one", "two", "three"],
            "url": "https://github.com/Kong/kong-mesh",
            "file": "~/Desktop",
            "auto": "~/Pictures"
        }));

    // Output the item to Alfred!
    Output::new()
        .rerun(Duration::from_secs(1))
        .skip_knowledge(false)
        .items([kuma, kong_mesh])
        .write(BufWriter::new(io::stdout()))?;

    Ok(())
}
