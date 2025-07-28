use twitter_v2::id::NumericId;

use super::*;

pub struct CampaignAnnouncement {
    app: App,
}

impl CampaignAnnouncement {
    pub fn new(app: &App) -> Self {
        Self { app: app.clone() }
    }

    pub async fn send_pending_announcements(&self) -> AsamiResult<()> {
        self.send_pending_from(
            "asami_club",
            |a| a.x_announcement_id_en_is_set(false),
            |u, id| u.x_announcement_id_en(id),
            &CAMPAIGN_ALERTS_EN,
        )
        .await?;
        self.send_pending_from(
            "asami_club_es",
            |a| a.x_announcement_id_es_is_set(false),
            |u, id| u.x_announcement_id_es(id),
            &CAMPAIGN_ALERTS_ES,
        )
        .await?;
        Ok(())
    }

    pub async fn send_pending_from(
        &self,
        username: &str,
        select: impl Fn(SelectCampaignHub) -> SelectCampaignHub,
        update: impl Fn(UpdateCampaignHub, Option<String>) -> UpdateCampaignHub,
        texts: &[&str],
    ) -> AsamiResult<()> {
        let Some(handle) = self.app.handle().select().username_eq(username.to_string()).optional().await? else {
            return Ok(());
        };

        let Ok(twitter) = handle.clone().x_api_client().await else {
            return Ok(());
        };

        let campaigns = select(self.app.campaign().select())
            .status_eq(CampaignStatus::Published)
            .budget_gt(u("10").encode_hex())
            .all()
            .await?;

        for campaign in campaigns {
            let idx = usize::try_from(*campaign.id()).unwrap_or(0) % 20;

            let post_result = twitter
                .post_tweet()
                .text(texts[idx].to_string())
                .quote_tweet_id(NumericId::from_str(&campaign.content_id()?)?)
                .send()
                .await;
            match post_result {
                Ok(post) => {
                    if let Some(post_id) = post.into_data().map(|t| t.id.to_string()) {
                        update(campaign.update(), Some(post_id.to_string())).save().await?;
                    } else {
                        self.app.fail("announcing_campaign", "no_poll_id_returned", ()).await;
                    }
                }
                Err(e) => {
                    self.app.fail("announcing_campaign", "no_post_result", format!("{e:?}")).await;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(
                self.app.settings.x.score_cooldown_seconds * 1000,
            ))
            .await;
        }

        Ok(())
    }
}

const CAMPAIGN_ALERTS_EN: [&str; 20] = [
    "🚨 New campaign just dropped! Take a moment to check it out—do you think it’s worth sharing with your audience?\nJoin https://asami.club",

    "🔥 A fresh campaign is live! If the message resonates with you, amplify it. Your voice can make the difference.\nJoin https://asami.club",

    "👀 New campaign alert! Would your followers appreciate this project? Think it over before deciding to share.\nJoin https://asami.club",

    "✨ Just in: a new campaign looking for visibility. Could your repost be what gets it seen?\nJoin https://asami.club",

    "📣 A project just launched a campaign. Take a sec to evaluate—would you want your friends to see this?\nJoin https://asami.club",

    "🧃 Something new is on the radar. Trust your taste—does this feel worthy of your support?\nJoin https://asami.club",

    "💥 New campaign out now. Look it over and ask yourself: does this deserve a boost?\nJoin https://asami.club",

    "🌀 Time to filter! A campaign just dropped. Does it match your values? Think before reposting.\nJoin https://asami.club",

    "🎯 A fresh post is calling for advocates. Would this matter to your followers?\nJoin https://asami.club",

    "📊 A new campaign is live. Could your signal help it break through the noise?\nJoin https://asami.club",

    "👑 Campaign in the feed! Your voice helps decide what’s seen. Would you vouch for this?\nJoin https://asami.club",

    "🔍 Pause and assess: is this something worth bringing to your community?\nJoin https://asami.club",

    "💫 Just launched: a new campaign hoping to find its audience. Think it’s worth sharing?\nJoin https://asami.club",

    "🎲 A campaign just went live. Does it align with your perspective? Your choice matters.\nJoin https://asami.club",

    "🚀 Let’s go! A new campaign is asking for visibility. Give it a look—does it earn your signal?\nJoin https://asami.club",

    "🧠 Think fast—but not too fast. A new campaign is up. Trust your judgment before sharing.\nJoin https://asami.club",

    "📬 Campaign drop detected. Before boosting, ask yourself—does this reflect your values?\nJoin https://asami.club",

    "🎉 Fresh campaign just hit the feed. You don’t need to act immediately—just read and decide.\nJoin https://asami.club",

    "💌 A project is reaching out. Is its message something you’d proudly stand behind?\nJoin https://asami.club",

    "📢 A campaign has launched. You help shape what matters—choose carefully.\nJoin https://asami.club",
];

const CAMPAIGN_ALERTS_ES: [&str; 20] = [
    "🚨 ¡Nueva campaña publicada! Tómate un momento para verla. ¿Crees que vale la pena compartirla con tu audiencia?\nÚnete en https://asami.club",

    "🔥 ¡Una nueva campaña está activa! Si el mensaje conecta contigo, dale difusión. Tu voz puede marcar la diferencia.\nÚnete en https://asami.club",

    "👀 Alerta de campaña nueva. ¿Le interesaría esto a tu comunidad? Piensa bien antes de compartir.\nÚnete en https://asami.club",

    "✨ Acaba de salir una campaña nueva. ¿Tu repost podría ayudarla a ser vista?\nÚnete en https://asami.club",

    "📣 Un proyecto lanzó una campaña. ¿Te parece valiosa? ¿La compartirías con tus seguidores?\nÚnete en https://asami.club",

    "🧃 Hay algo nuevo en el radar. Confía en tu criterio: ¿vale la pena amplificar este mensaje?\nÚnete en https://asami.club",

    "💥 Nueva campaña disponible. Échale un vistazo y pregúntate: ¿merece un impulso de tu parte?\nÚnete en https://asami.club",

    "🌀 ¡Hora de filtrar! Hay una campaña nueva. ¿Está alineada con lo que tú valoras?\nÚnete en https://asami.club",

    "🎯 Un nuevo mensaje busca divulgadores. ¿Crees que tu audiencia debería verlo?\nÚnete en https://asami.club",

    "📊 Una campaña acaba de salir. ¿Tu señal podría ayudarla a destacarse?\nÚnete en https://asami.club",

    "👑 Campaña nueva en circulación. Tu voz le da fuerza. ¿Te parece que vale la pena?\nÚnete en https://asami.club",

    "🔍 Haz una pausa y evalúa: ¿esto es algo que vale la pena compartir?\nÚnete en https://asami.club",

    "💫 Acaba de lanzarse una nueva campaña. ¿Sientes que merece ser difundida?\nÚnete en https://asami.club",

    "🎲 Hay una campaña nueva. ¿Está en línea con tus valores? Tú decides si amplificarla o no.\nÚnete en https://asami.club",

    "🚀 ¡Vamos! Una campaña recién llegó al feed. Si te representa, dale ese empujón.\nÚnete en https://asami.club",

    "🧠 Piensa rápido… pero no tanto. Una nueva campaña apareció. Confía en tu criterio antes de compartir.\nÚnete en https://asami.club",

    "📬 Detectamos una campaña nueva. Antes de amplificarla, pregúntate: ¿esto refleja lo que tú valoras?\nÚnete en https://asami.club",

    "🎉 Una nueva campaña está al aire. No tienes que decidir ya—solo mírala y elige si va contigo.\nÚnete en https://asami.club",

    "💌 Un proyecto está alzando la voz. ¿Es un mensaje que compartirías con orgullo?\nÚnete en https://asami.club",

    "📢 Nueva campaña disponible. Tú ayudas a decidir qué merece ser visto. Elige con cuidado.\nÚnete en https://asami.club",
];
