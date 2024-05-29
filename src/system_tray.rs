use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

pub enum SystemTrayEvent {
    Quit,
}

enum Message {
    Red,
    Green,
    Quit,
}

pub struct SystemTray;

impl SystemTray {
    pub fn new() -> Self {
        Self
    }

    pub fn on<F>(&self, event_handler: F) -> ()
    where
        F: FnOnce(SystemTrayEvent),
    {
    }
}

pub fn tray_main() {
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    })
    .unwrap();

    // tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let red_tx = tx.clone();
    tray.add_menu_item("Red", move || {
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    let green_tx = tx.clone();
    tray.add_menu_item("Green", move || {
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    // tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    println!("ok!");
    tray.inner_mut().display();

    loop {
        println!("yes!");
        for m in rx.try_iter() {
            match m {
                Message::Quit => {
                    println!("Quit");
                    break;
                }
                Message::Red => {
                    println!("Red");
                    // tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                    //     .unwrap();
                }
                Message::Green => {
                    println!("Green");
                    // tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
                    //     .unwrap()
                }
                _ => {}
            }
        }
    }
}
