use dashmap::DashMap;

pub struct Cache {
    users: DashMap<u64, UserData>,
    guilds: DashMap<u64, GuildData>,
    dms: DashMap<u64, ChannelData>,
}

pub struct UserData {
    name: String,
    discriminator: u16,
    avatar: Option<File>,
    bot: bool,
}

pub struct GuildData {
    name: String,
    splash: Option<File>,
    banner: Option<File>,
    owner_id: u64,
    icon: Option<File>,
    members: DashMap<u64, MemberData>,
    roles: DashMap<u64, RoleData>,
    channels: DashMap<u64, ChannelData>,
}

pub struct ChannelData {
    name: String,
    kind: ChannelKind,
    position: i64,
    topic: Option<String>,
    nsfw: bool,
    slow_mode_rate: Option<u64>,
    user_limit: Option<u64>,
}

pub enum ChannelKind {
    Text,
    Private,
    Voice,
    Group,
    Category,
    News,
    Store,
}

pub struct RoleData {
    color: (u8, u8, u8),
    hoist: bool,
    name: String,
    position: i64,
}

pub struct MemberData {
    user_id: u64,
    nickname: Option<String>,
    roles: Vec<u64>,
}

pub struct File(Vec<u8>);
