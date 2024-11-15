pub mod screen {
    use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
    use druid::piet::TextStorage;
    use druid::widget::{Button, Flex, TextBox};
    use crate::connexion::client::client::TcpConnexion;
    use super::super::super::connexion::client::client;

    #[derive(Clone, Data, Lens)]
    pub struct ClientData {
        name: String
    }

    fn ui_builder() -> impl Widget<ClientData> {
        let mut col = Flex::column();
        col.add_child(TextBox::new().lens(ClientData::name).padding(10.0));

        let connect_button = Button::new("Connect".to_string())
            .padding(3.0)
            .on_click(|_ctx, data: &mut ClientData, _env| {
                let mut client = TcpConnexion::new("localhost:5444".to_string()).unwrap();
                client.send_message("Hello".as_str());
            });

        col.add_child(connect_button);

        Flex::column().with_child(col)
    }

    pub fn new_window() {
        let main_window = WindowDesc::new(ui_builder());
        let data = ClientData {
            name: "Lulu".to_string()
        };
        AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data)
            .expect("Couldn't launch window");

    }
}