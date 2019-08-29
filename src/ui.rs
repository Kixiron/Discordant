use crate::backend::BackendMsg;
use tokio::sync::mpsc;

pub struct DecodedImageData(*mut u8, usize, i32, i32, i32);

unsafe impl Send for DecodedImageData {}

use gdk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Orientation, WindowPosition};
use serenity::{
    client::Client,
    http::GuildPagination,
    model::{
        channel::GuildChannel,
        channel::Message as ChatMessage,
        gateway::Ready,
        guild::{GuildInfo, Member, PartialGuild},
        id::{ChannelId, GuildId},
        user::CurrentUser,
    },
    prelude::*,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InitializationState {
    pub guilds: Vec<(PartialGuild, Vec<Member>, HashMap<ChannelId, GuildChannel>)>,
    pub user: CurrentUser,
}

fn rounded_image(pixbuf: gdk_pixbuf::Pixbuf, radius: f64) -> gtk::DrawingArea {
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

fn build_ui(application: &gtk::Application, state: &InitializationState) {
    let window = ApplicationWindow::new(application);

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
            // TODO
            /*
            if let Ok(mut data) = reqwest::get(&icon_url) {
                let mut buf = Vec::with_capacity(1024 * 64);
                use std::io::Read;
                data.read_to_end(&mut buf).unwrap();
                let buf = &buf[..];
                let pixbuf = bytes_to_pixbuf(&buf[..]);
                const RADIUS: f64 = 25.0;
                if let Some(pixbuf) = pixbuf {
                    if let Some(pixbuf) = pixbuf.scale_simple(
                        (RADIUS * 2.0) as _,
                        (RADIUS * 2.0) as _,
                        gdk_pixbuf::InterpType::Bilinear,
                    ) {
                        guild_row.add(&rounded_image(pixbuf, RADIUS));
                    }
                }
            }
            */
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
            // TODO
            /*
            if let Ok(mut data) = reqwest::get(&avatar_url) {
                let mut buf = Vec::with_capacity(1024 * 64);
                use std::io::Read;
                data.read_to_end(&mut buf).unwrap();
                let buf = &buf[..];
                let pixbuf = bytes_to_pixbuf(&buf[..]);
                const RADIUS: f64 = 15.0;
                if let Some(pixbuf) = pixbuf {
                    if let Some(pixbuf) = pixbuf.scale_simple(
                        (RADIUS * 2.0) as _,
                        (RADIUS * 2.0) as _,
                        gdk_pixbuf::InterpType::Bilinear,
                    ) {
                        member_row.add(&rounded_image(pixbuf, RADIUS));
                    }
                }
            }
            */
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
}

pub fn run(
    discord_receiver: std::sync::mpsc::Receiver<BackendMsg>,
    download_url_input: mpsc::Sender<String>,
    downloaded_images_output: mpsc::Receiver<DecodedImageData>,
) {
    while let initialization_state = discord_receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("timeout")
    {
        println!("{:?}", initialization_state);
        if let BackendMsg::Ready(_, initialization_state) = initialization_state {
            let args = std::env::args().collect::<Vec<_>>();
            let application =
                gtk::Application::new(Some("com.Discordant.Discordant"), Default::default())
                    .expect("Initialization failed...");
            application.connect_activate(move |app| {
                build_ui(app, &initialization_state);
            });
            application.run(&args);
            std::panic!();
        }
    }
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
