use crate::backend;
use futures::{
    channel::mpsc::{Receiver, Sender},
    sink::SinkExt,
    stream::StreamExt,
};
use gdk::prelude::ContextExt;
use gtk::{
    BoxExt, ContainerExt, GtkWindowExt, Inhibit, Orientation, ScrolledWindowExt, TextViewExt,
    WidgetExt, Window, WindowPosition, WindowType,
};
use relm::{connect, connect_stream, Relm, Update, Widget};
use relm_derive::Msg;
use serenity::model::{
    channel::GuildChannel,
    guild::{Member, PartialGuild},
    id::ChannelId,
    user::CurrentUser,
};
use std::collections::HashMap;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Win {
    window: Window,
    discord: backend::Discord,
    backend_recv: Receiver<backend::BackendMsg>,
    url_sender: Sender<String>,
    file_recv: Receiver<DecodedImageData>,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        ()
    }

    fn update(&mut self, event: Self::Msg) {
        println!("here");
        if let Some(msg) = futures::executor::block_on(self.backend_recv.next()) {
            match msg {
                msg => self.window.title = format!("{:?}", msg),
            }
        }

        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let (discord, mut backend_recv, mut url_sender, mut file_recv) = backend::main(
            std::env::var("DISCORD_TOKEN").expect("Missing token env var (DISCORD_TOKEN)"),
        );

        let state = loop {
            if let Some(backend::BackendMsg::Ready(_, initialization_state)) =
                futures::executor::block_on(backend_recv.next())
            {
                break initialization_state;
            }
        };

        let window = Window::new(WindowType::Toplevel);

        window.set_title("Discordant");
        window.set_border_width(10);
        window.set_position(WindowPosition::Center);
        window.maximize();

        let topmost_container = gtk::Box::new(Orientation::Horizontal, 0);
        let leftmost_guild_list = gtk::Box::new(Orientation::Vertical, 2);
        let left_channel_list = gtk::Box::new(Orientation::Vertical, 2);
        let middle_chat = gtk::Box::new(Orientation::Vertical, 2);
        let rightmost_member_list = gtk::Box::new(Orientation::Vertical, 2);
        topmost_container.pack_start(&leftmost_guild_list, false, false, 0);
        topmost_container.pack_start(&left_channel_list, false, false, 0);
        topmost_container.pack_start(&middle_chat, true, true, 0);
        topmost_container.pack_start(&rightmost_member_list, false, false, 0);

        let guild_list = gtk::ListBox::new();
        for guild in state.guilds.iter() {
            let guild_row = gtk::Box::new(Orientation::Horizontal, 0);
            if let Some(icon_url) = guild.0.icon_url() {
                if let Err(err) = futures::executor::block_on(url_sender.send(icon_url)) {
                    eprintln!("URL Send Error: {:?}", err);
                } else {
                    if let Some(file) = futures::executor::block_on(file_recv.next()) {
                        const RADIUS: f64 = 25.0;

                        if let Some(pixbuf) = image_data_to_pixbuf(file).scale_simple(
                            (RADIUS * 2.0) as _,
                            (RADIUS * 2.0) as _,
                            gdk_pixbuf::InterpType::Bilinear,
                        ) {
                            guild_row.add(&rounded_image(pixbuf, RADIUS));
                        }
                    }
                }
            }
            guild_row.add(&gtk::Label::new(Some(&guild.0.name)));
            guild_list.add(&guild_row);
        }
        guild_list.show();
        leftmost_guild_list.pack_start(&guild_list, true, true, 0);

        let channel_list = gtk::ListBox::new();
        for (_channel_id, channel) in &state.guilds[0].2 {
            channel_list.add(&gtk::Label::new(Some(&channel.name)));
        }
        channel_list.show();
        left_channel_list.pack_start(&channel_list, true, true, 0);

        let member_list = gtk::ListBox::new();
        for member in &state.guilds[0].1 {
            let member_row = gtk::Box::new(Orientation::Horizontal, 0);
            let user = member.user.read();
            if let Some(avatar_url) = user.avatar_url() {
                if let Err(err) = futures::executor::block_on(url_sender.send(avatar_url)) {
                    eprintln!("URL Send Error: {:?}", err);
                } else {
                    if let Some(file) = futures::executor::block_on(file_recv.next()) {
                        const RADIUS: f64 = 25.0;

                        if let Some(pixbuf) = image_data_to_pixbuf(file).scale_simple(
                            (RADIUS * 2.0) as _,
                            (RADIUS * 2.0) as _,
                            gdk_pixbuf::InterpType::Bilinear,
                        ) {
                            member_row.add(&rounded_image(pixbuf, RADIUS));
                        }
                    }
                }
            }
            member_row.pack_start(&gtk::Label::new(Some(&user.name)), true, true, 0);
            member_list.add(&member_row);
        }
        member_list.show();
        rightmost_member_list.pack_start(&member_list, true, true, 0);

        let text_view = gtk::TextView::new();
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        text_view.set_editable(true);
        text_view.set_can_focus(true);
        let scrolled_text_view = gtk::ScrolledWindow::new(
            gtk::NONE_ADJUSTMENT,
            Some(&gtk::Adjustment::new(0.1, 0.1, 1.0, 0.1, 0.1, 0.1)),
        );
        scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled_text_view.add(&text_view);

        middle_chat.pack_start(&scrolled_text_view, true, true, 0);
        let text_view = gtk::TextView::new();
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        text_view.set_editable(true);
        text_view.set_can_focus(true);
        let scrolled_text_view = gtk::ScrolledWindow::new(
            gtk::NONE_ADJUSTMENT,
            Some(&gtk::Adjustment::new(0.1, 0.1, 1.0, 0.1, 0.1, 0.1)),
        );
        scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled_text_view.add(&text_view);

        middle_chat.pack_start(&scrolled_text_view, true, true, 0);

        window.add(&topmost_container);
        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        Self {
            window,
            discord,
            backend_recv,
            url_sender,
            file_recv,
        }
    }
}

pub struct DecodedImageData(*mut u8, usize, i32, i32, i32);

unsafe impl Send for DecodedImageData {}

#[derive(Debug, Clone)]
pub struct InitializationState {
    pub guilds: Vec<(PartialGuild, Vec<Member>, HashMap<ChannelId, GuildChannel>)>,
    pub user: CurrentUser,
}

pub fn rounded_image(pixbuf: gdk_pixbuf::Pixbuf, radius: f64) -> gtk::DrawingArea {
    let img = gtk::DrawingArea::new();
    img.connect_draw(move |da, g| {
        da.set_size_request(50, 50);
        g.set_source_pixbuf(&pixbuf, 1.0, 1.0);
        g.arc(radius, radius, radius, 0.0, 2.0 * std::f64::consts::PI);
        g.clip();
        g.rectangle(0.0, 0.0, radius * 2.0, radius * 2.0);
        g.fill();
        g.paint();
        gtk::Inhibit(false)
    });
    img
}

pub fn decode_webp(input_bytes: &[u8]) -> Option<DecodedImageData> {
    let image_data = {
        const BYTES_PER_PX: i32 = 3;
        let mut width = 0;
        let mut height = 0;
        let input_len = input_bytes.len();
        let is_valid = unsafe {
            libwebp_sys::WebPGetInfo(input_bytes.as_ptr(), input_len, &mut width, &mut height) != 0
        };
        if !is_valid {
            // TODO: gif
            None
        } else {
            let decoded_image = unsafe {
                libwebp_sys::WebPDecodeRGB(input_bytes.as_ptr(), input_len, &mut width, &mut height)
            };
            let len = width * height * BYTES_PER_PX;
            let stride = height * BYTES_PER_PX;
            Some(DecodedImageData(
                decoded_image,
                len as usize,
                width,
                height,
                stride,
            ))
        }
    };
    image_data
}

fn image_data_to_pixbuf(
    DecodedImageData(image_data, len, width, height, stride): DecodedImageData,
) -> gdk_pixbuf::Pixbuf {
    let buf_slice = unsafe { std::slice::from_raw_parts(image_data, len) };
    // TODO: Avoid copy making glib::Bytes free using WebPFree
    let pixbuf = gdk_pixbuf::Pixbuf::new_from_bytes(
        &glib::Bytes::from_owned(buf_slice.to_vec()),
        gdk_pixbuf::Colorspace::Rgb,
        false,
        8,
        width,
        height,
        stride,
    );
    // We want to drop the slice before freeing the ptr
    #[allow(clippy::drop_ref)]
    drop(buf_slice);
    unsafe { libwebp_sys::WebPFree(image_data as _) };
    pixbuf
}
