use dashmap::DashMap;
use serde_json::Value;
use serenity::{
    client::{
        bridge::{
            gateway::{event::ShardStageUpdateEvent, ShardManager},
            voice::ClientVoiceManager,
        },
        Client, Context, EventHandler,
    },
    http::raw::Http,
    model::{
        channel::{Channel, ChannelCategory, GuildChannel, Message, PrivateChannel, Reaction},
        event::{
            ChannelPinsUpdateEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent,
            TypingStartEvent, VoiceServerUpdateEvent,
        },
        gateway::{Presence, Ready},
        guild::{Emoji, Guild, Member, PartialGuild, Role},
        id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId},
        user::{CurrentUser, User},
        voice::VoiceState,
    },
    prelude::{Mutex, RwLock, TypeMapKey},
};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
};

unsafe impl Send for BackendMsg {}
unsafe impl Sync for BackendMsg {}

struct Handler;

impl EventHandler for Handler {
    fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::CacheReady(guilds))
            .expect("Failed to send backend message");
    }
    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelCreate(channel))
            .expect("Failed to send backend message");
    }
    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::CategoryCreate(category))
            .expect("Failed to send backend message");
    }
    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::CategoryDelete(category))
            .expect("Failed to send backend message");
    }
    fn private_channel_create(&self, ctx: Context, channel: Arc<RwLock<PrivateChannel>>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::PrivateChannelCreate(channel))
            .expect("Failed to send backend message");
    }
    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelDelete(channel))
            .expect("Failed to send backend message");
    }
    fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelPinsUpdate(pin))
            .expect("Failed to send backend message");
    }
    fn channel_recipient_addition(&self, ctx: Context, group_id: ChannelId, user: User) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelRecipientAdd(group_id, user))
            .expect("Failed to send backend message");
    }
    fn channel_recipient_removal(&self, ctx: Context, group_id: ChannelId, user: User) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelRecipientRm(group_id, user))
            .expect("Failed to send backend message");
    }
    fn channel_update(&self, ctx: Context, old: Option<Channel>, new: Channel) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ChannelUpdate(old, new))
            .expect("Failed to send backend message");
    }
    fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildBanAdd(guild_id, banned_user))
            .expect("Failed to send backend message");
    }
    fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildBanRm(guild_id, unbanned_user))
            .expect("Failed to send backend message");
    }
    fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildCreate(guild, is_new))
            .expect("Failed to send backend message");
    }
    fn guild_delete(
        &self,
        ctx: Context,
        incomplete: PartialGuild,
        full: Option<Arc<RwLock<Guild>>>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildDel(incomplete, full))
            .expect("Failed to send backend message");
    }
    fn guild_emojis_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        current_state: HashMap<EmojiId, Emoji>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildEmojiUpdate(guild_id, current_state))
            .expect("Failed to send backend message");
    }
    fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildIntegrationsUpdate(guild_id))
            .expect("Failed to send backend message");
    }
    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildMemberAdd(guild_id, new_member))
            .expect("Failed to send backend message");
    }
    fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data: Option<Member>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildMemberRm(guild_id, user, member_data))
            .expect("Failed to send backend message");
    }
    fn guild_member_update(&self, ctx: Context, old: Option<Member>, new: Member) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildMemberUpdate(old, new))
            .expect("Failed to send backend message");
    }

    fn guild_members_chunk(
        &self,
        ctx: Context,
        guild_id: GuildId,
        offline_members: HashMap<UserId, Member>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildMembersOffline(guild_id, offline_members))
            .expect("Failed to send backend message");
    }
    fn guild_role_create(&self, ctx: Context, guild_id: GuildId, new: Role) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildRoleAdd(guild_id, new))
            .expect("Failed to send backend message");
    }
    fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        role_id: RoleId,
        role_data: Option<Role>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildRoleRm(guild_id, role_id, role_data))
            .expect("Failed to send backend message");
    }
    fn guild_role_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        old_data: Option<Role>,
        new: Role,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildRoleUpdate(guild_id, old_data, new))
            .expect("Failed to send backend message");
    }
    fn guild_unavailable(&self, ctx: Context, guild_id: GuildId) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildUnavailable(guild_id))
            .expect("Failed to send backend message");
    }
    fn guild_update(&self, ctx: Context, old_data: Option<Arc<RwLock<Guild>>>, new: PartialGuild) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::GuildUpdate(old_data, new))
            .expect("Failed to send backend message");
    }
    fn message(&self, ctx: Context, new_message: Message) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        if let Err(err) = sender.0.send(BackendMsg::MessageAdd(new_message)) {
            println!("{:?}", err);
        }
    }
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        if let Err(err) = sender.0.send(BackendMsg::MessageRm(channel_id, message_id)) {
            println!("{:?}", err);
        }
    }
    fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_messages: Vec<MessageId>,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::MessageRmBulk(channel_id, deleted_messages))
            .expect("Failed to send backend message");
    }
    fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::MessageUpdate(old, new, event))
            .expect("Failed to send backend message");
    }
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ReactionAdd(reaction))
            .expect("Failed to send backend message");
    }
    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ReactionRm(reaction))
            .expect("Failed to send backend message");
    }
    fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ReactionRmAll(channel_id, message_id))
            .expect("Failed to send backend message");
    }
    fn presence_replace(&self, ctx: Context, presences: Vec<Presence>) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::PresenceReplace(presences))
            .expect("Failed to send backend message");
    }
    fn presence_update(&self, ctx: Context, data: PresenceUpdateEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::PresenceUpdate(data))
            .expect("Failed to send backend message");
    }
    fn ready(&self, ctx: Context, data: Ready) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::Ready(data))
            .expect("Failed to send backend message");
    }
    fn resume(&self, ctx: Context, data: ResumedEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::Resume(data))
            .expect("Failed to send backend message");
    }
    fn shard_stage_update(&self, ctx: Context, data: ShardStageUpdateEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::ShardStageUpdate(data))
            .expect("Failed to send backend message");
    }
    fn typing_start(&self, ctx: Context, data: TypingStartEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::TypingStart(data))
            .expect("Failed to send backend message");
    }
    fn user_update(&self, ctx: Context, old: CurrentUser, new: CurrentUser) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::UserUpdate(old, new))
            .expect("Failed to send backend message");
    }
    fn voice_server_update(&self, ctx: Context, data: VoiceServerUpdateEvent) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::VoiceServerUpdate(data))
            .expect("Failed to send backend message");
    }
    fn voice_state_update(
        &self,
        ctx: Context,
        guild_id: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::VoiceStateUpdate(guild_id, old, new))
            .expect("Failed to send backend message");
    }
    fn webhook_update(&self, ctx: Context, guild_id: GuildId, channel_id: ChannelId) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::WebhookUpdate(guild_id, channel_id))
            .expect("Failed to send backend message");
    }
    fn unknown(&self, ctx: Context, name: String, raw: Value) {
        let context = ctx.data.read();
        let sender = context.get::<SenderKey>().expect("Expected Sender");

        sender
            .0
            .send(BackendMsg::Unknown(name, raw))
            .expect("Failed to send backend message");
    }
}

#[derive(Clone, Debug)]
pub enum BackendMsg {
    CacheReady(Vec<GuildId>),
    ChannelCreate(Arc<RwLock<GuildChannel>>),
    CategoryCreate(Arc<RwLock<ChannelCategory>>),
    CategoryDelete(Arc<RwLock<ChannelCategory>>),
    PrivateChannelCreate(Arc<RwLock<PrivateChannel>>),
    ChannelDelete(Arc<RwLock<GuildChannel>>),
    ChannelPinsUpdate(ChannelPinsUpdateEvent),
    ChannelRecipientAdd(ChannelId, User),
    ChannelRecipientRm(ChannelId, User),
    ChannelUpdate(Option<Channel>, Channel),
    GuildBanAdd(GuildId, User),
    GuildBanRm(GuildId, User),
    GuildCreate(Guild, bool),
    GuildDel(PartialGuild, Option<Arc<RwLock<Guild>>>),
    GuildEmojiUpdate(GuildId, HashMap<EmojiId, Emoji>),
    GuildIntegrationsUpdate(GuildId),
    GuildMemberAdd(GuildId, Member),
    GuildMemberRm(GuildId, User, Option<Member>),
    GuildMemberUpdate(Option<Member>, Member),
    GuildMembersOffline(GuildId, HashMap<UserId, Member>),
    GuildRoleAdd(GuildId, Role),
    GuildRoleRm(GuildId, RoleId, Option<Role>),
    GuildRoleUpdate(GuildId, Option<Role>, Role),
    GuildUnavailable(GuildId),
    GuildUpdate(Option<Arc<RwLock<Guild>>>, PartialGuild),
    MessageAdd(Message),
    MessageRm(ChannelId, MessageId),
    MessageRmBulk(ChannelId, Vec<MessageId>),
    MessageUpdate(Option<Message>, Option<Message>, MessageUpdateEvent),
    ReactionAdd(Reaction),
    ReactionRm(Reaction),
    ReactionRmAll(ChannelId, MessageId),
    PresenceReplace(Vec<Presence>),
    PresenceUpdate(PresenceUpdateEvent),
    Ready(Ready),
    Resume(ResumedEvent),
    ShardStageUpdate(ShardStageUpdateEvent),
    TypingStart(TypingStartEvent),
    UserUpdate(CurrentUser, CurrentUser),
    VoiceServerUpdate(VoiceServerUpdateEvent),
    VoiceStateUpdate(Option<GuildId>, Option<VoiceState>, VoiceState),
    WebhookUpdate(GuildId, ChannelId),
    Unknown(String, Value),
}

#[derive(Clone, Debug)]
#[repr(transparent)]
struct SendWrap(Sender<BackendMsg>);

unsafe impl Send for SendWrap {}
unsafe impl Sync for SendWrap {}

struct SenderKey;
impl TypeMapKey for SenderKey {
    type Value = Arc<SendWrap>;
}

pub struct Discord {
    http: Arc<Http>,
    shard_manager: Arc<Mutex<ShardManager>>,
    voice_manager: Arc<Mutex<ClientVoiceManager>>,
}

impl Discord {
    #[inline]
    pub fn spawn(token: impl AsRef<str>) -> (Self, Receiver<BackendMsg>) {
        let mut client = Client::new(token, Handler).expect("Err creating client");

        let (sender, receiver) = mpsc::channel::<BackendMsg>();
        let discord = Self {
            http: Arc::clone(&client.cache_and_http.http),
            shard_manager: Arc::clone(&client.shard_manager),
            voice_manager: Arc::clone(&client.voice_manager),
        };

        {
            let mut data = client.data.write();

            data.insert::<SenderKey>(Arc::new(SendWrap(sender)));
        }

        thread::Builder::new()
            .name("Backend".to_string())
            .spawn(move || {
                if let Err(err) = client.start() {
                    println!("Client error: {:?}", err);
                }
            })
            .expect("Failed to spawn Serenity thread");

        (discord, receiver)
    }

    #[inline]
    pub fn add_group_member(&self, group_id: u64, user_id: u64) -> Result<(), serenity::Error> {
        (*self.http).add_group_recipient(group_id, user_id)
    }

    #[inline]
    pub fn add_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<(), serenity::Error> {
        (*self.http).add_member_role(guild_id, user_id, role_id)
    }

    #[inline]
    pub fn send_message(&self, channel_id: u64, content: &str) -> Result<Message, serenity::Error> {
        (*self.http).send_message(channel_id, &serde_json::json!({ "content": content }))
    }

    #[inline]
    pub fn restart(&mut self) {
        let mut manager = self.shard_manager.lock();
        for shard in &manager.shards_instantiated() {
            manager.restart(*shard);
        }
    }
}

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

pub struct File;
