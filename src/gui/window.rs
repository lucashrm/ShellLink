pub mod screen {
    use std::{io, thread};
    use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
    use druid::piet::TextStorage;
    use druid::widget::{Button, Flex, TextBox};
    use crate::connexion::client::client::TcpConnexion;

    #[derive(Clone, Data, Lens)]
    pub struct ClientData {
        name: String,
        message: String
    }

    fn ui_builder() -> impl Widget<ClientData> {
        let mut col = Flex::column();
        col.add_child(TextBox::new().lens(ClientData::name).padding(10.0));

        let connect_button = Button::new("Connect".to_string())
            .padding(3.0)
            .on_click(|_ctx, data: &mut ClientData, _env| {
                let mut client = TcpConnexion::new("localhost:5444".to_string()).unwrap();
                client.send_message(data.name.as_str());
                thread::spawn(move || {
                    loop {
                        let mut message = String::new();

                        io::stdin()
                            .read_line(&mut message)
                            .expect("Failed to read line");

                        client.send_message(message.as_str());
                    }
                });
            });

        col.add_child(connect_button);

        col.add_child(TextBox::new().lens(ClientData::message).padding(10.0));

        Flex::column().with_child(col)
    }

    pub fn new_window() -> Result<(), &'static str>{
        let main_window = WindowDesc::new(ui_builder());
        let data = ClientData {
            name: "Lulu".to_string(),
            message: "".to_string()
        };
        match AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data) {
            Err(_) => Err("Failed to launch window"),
            Ok(_) => Ok(())
        }
    }
}