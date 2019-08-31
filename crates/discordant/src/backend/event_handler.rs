use super::{BackendMsg, SenderKey};
use serde_json::Value;
use serenity::{
    client::{bridge::gateway::event::ShardStageUpdateEvent, Context, EventHandler},
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
    prelude::RwLock,
};
use std::{collections::HashMap, sync::Arc};

pub struct Handler;

impl EventHandler for Handler {
    fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::CacheReady(guilds))
            .expect("Failed to send backend message");
    }
    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelCreate(channel))
            .expect("Failed to send backend message");
    }
    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::CategoryCreate(category))
            .expect("Failed to send backend message");
    }
    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::CategoryDelete(category))
            .expect("Failed to send backend message");
    }
    fn private_channel_create(&self, ctx: Context, channel: Arc<RwLock<PrivateChannel>>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::PrivateChannelCreate(channel))
            .expect("Failed to send backend message");
    }
    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelDelete(channel))
            .expect("Failed to send backend message");
    }
    fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelPinsUpdate(pin))
            .expect("Failed to send backend message");
    }
    fn channel_recipient_addition(&self, ctx: Context, group_id: ChannelId, user: User) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelRecipientAdd(group_id, user))
            .expect("Failed to send backend message");
    }
    fn channel_recipient_removal(&self, ctx: Context, group_id: ChannelId, user: User) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelRecipientRm(group_id, user))
            .expect("Failed to send backend message");
    }
    fn channel_update(&self, ctx: Context, old: Option<Channel>, new: Channel) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ChannelUpdate(old, new))
            .expect("Failed to send backend message");
    }
    fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildBanAdd(guild_id, banned_user))
            .expect("Failed to send backend message");
    }
    fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildBanRm(guild_id, unbanned_user))
            .expect("Failed to send backend message");
    }
    fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildCreate(guild, is_new))
            .expect("Failed to send backend message");
    }
    fn guild_delete(
        &self,
        ctx: Context,
        incomplete: PartialGuild,
        full: Option<Arc<RwLock<Guild>>>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildDel(incomplete, full))
            .expect("Failed to send backend message");
    }
    fn guild_emojis_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        current_state: HashMap<EmojiId, Emoji>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildEmojiUpdate(guild_id, current_state))
            .expect("Failed to send backend message");
    }
    fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildIntegrationsUpdate(guild_id))
            .expect("Failed to send backend message");
    }
    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildMemberAdd(guild_id, new_member))
            .expect("Failed to send backend message");
    }
    fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data: Option<Member>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildMemberRm(guild_id, user, member_data))
            .expect("Failed to send backend message");
    }
    fn guild_member_update(&self, ctx: Context, old: Option<Member>, new: Member) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildMemberUpdate(old, new))
            .expect("Failed to send backend message");
    }

    fn guild_members_chunk(
        &self,
        ctx: Context,
        guild_id: GuildId,
        offline_members: HashMap<UserId, Member>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildMembersOffline(guild_id, offline_members))
            .expect("Failed to send backend message");
    }
    fn guild_role_create(&self, ctx: Context, guild_id: GuildId, new: Role) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildRoleAdd(guild_id, new))
            .expect("Failed to send backend message");
    }
    fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        role_id: RoleId,
        role_data: Option<Role>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildRoleRm(guild_id, role_id, role_data))
            .expect("Failed to send backend message");
    }
    fn guild_role_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        old_data: Option<Role>,
        new: Role,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildRoleUpdate(guild_id, old_data, new))
            .expect("Failed to send backend message");
    }
    fn guild_unavailable(&self, ctx: Context, guild_id: GuildId) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildUnavailable(guild_id))
            .expect("Failed to send backend message");
    }
    fn guild_update(&self, ctx: Context, old_data: Option<Arc<RwLock<Guild>>>, new: PartialGuild) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::GuildUpdate(old_data, new))
            .expect("Failed to send backend message");
    }
    fn message(&self, ctx: Context, new_message: Message) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::MessageAdd(new_message))
            .expect("Failed to send backend message");
    }
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::MessageRm(channel_id, message_id))
            .expect("Failed to send backend message");
    }
    fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_messages: Vec<MessageId>,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::MessageRmBulk(channel_id, deleted_messages))
            .expect("Failed to send backend message");
    }
    fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::MessageUpdate(old, new, event))
            .expect("Failed to send backend message");
    }
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ReactionAdd(reaction))
            .expect("Failed to send backend message");
    }
    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ReactionRm(reaction))
            .expect("Failed to send backend message");
    }
    fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ReactionRmAll(channel_id, message_id))
            .expect("Failed to send backend message");
    }
    fn presence_replace(&self, ctx: Context, presences: Vec<Presence>) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::PresenceReplace(presences))
            .expect("Failed to send backend message");
    }
    fn presence_update(&self, ctx: Context, data: PresenceUpdateEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::PresenceUpdate(data))
            .expect("Failed to send backend message");
    }

    fn ready(&self, ctx: Context, data: Ready) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        let user = ctx.http.get_current_user().unwrap();
        let guilds = ctx
            .http
            .get_guilds(&serenity::http::GuildPagination::After(GuildId(0)), 100)
            .unwrap();
        let http = &ctx.http;
        let guilds = guilds
            .into_iter()
            .map(|g| g.id.to_partial_guild(http).unwrap())
            .map(|pg| {
                let channels = pg.channels(http).unwrap();
                let members = pg.members(http, Some(1000), None).unwrap();
                (pg, members, channels)
            })
            .collect::<Vec<_>>();

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::Ready(
                data,
                crate::ui::InitializationState { guilds, user },
            ))
            .expect("Failed to send backend message");
    }

    fn resume(&self, ctx: Context, data: ResumedEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::Resume(data))
            .expect("Failed to send backend message");
    }
    fn shard_stage_update(&self, ctx: Context, data: ShardStageUpdateEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::ShardStageUpdate(data))
            .expect("Failed to send backend message");
    }
    fn typing_start(&self, ctx: Context, data: TypingStartEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::TypingStart(data))
            .expect("Failed to send backend message");
    }
    fn user_update(&self, ctx: Context, old: CurrentUser, new: CurrentUser) {
        let mut sender = {
            let context = ctx.data.read();
            context
                .get::<SenderKey>()
                .expect("Expected Sender")
                .clone()
                .clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::UserUpdate(old, new))
            .expect("Failed to send backend message");
    }
    fn voice_server_update(&self, ctx: Context, data: VoiceServerUpdateEvent) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::VoiceServerUpdate(data))
            .expect("Failed to send backend message");
    }
    fn voice_state_update(
        &self,
        ctx: Context,
        guild_id: Option<GuildId>,
        old: Option<VoiceState>,
        new: VoiceState,
    ) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::VoiceStateUpdate(guild_id, old, new))
            .expect("Failed to send backend message");
    }
    fn webhook_update(&self, ctx: Context, guild_id: GuildId, channel_id: ChannelId) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::WebhookUpdate(guild_id, channel_id))
            .expect("Failed to send backend message");
    }
    fn unknown(&self, ctx: Context, name: String, raw: Value) {
        let mut sender = {
            let context = ctx.data.read();
            context.get::<SenderKey>().expect("Expected Sender").clone()
        };

        Arc::make_mut(&mut sender)
            .0
            .try_send(BackendMsg::Unknown(name, raw))
            .expect("Failed to send backend message");
    }
}
