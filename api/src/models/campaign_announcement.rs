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

            let text = texts[idx]
                .replace(
                    "{rate}",
                    &format!(
                        "{:.2}",
                        wei_to_decimal_safe(campaign.price_per_point_u256() * wei("100"))?
                    ),
                )
                .replace(
                    "{min}",
                    &format!("{:.2}", wei_to_decimal_safe(campaign.min_individual_reward_u256())?),
                )
                .replace(
                    "{max}",
                    &format!("{:.2}", wei_to_decimal_safe(campaign.max_individual_reward_u256())?),
                );

            let post_result = twitter
                .post_tweet()
                .text(text)
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
    "🚨 Campaign just dropped! Think your audience will love it? Take a moment, judge it, and decide if it deserves your repost. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🔥 A new campaign is live! Are you ready to turn it into a trend—or will you pass this time? The choice is yours. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "👀 New campaign alert! Does it speak to you? Is it something your followers would jump on? Take a sec to evaluate. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "✨ Just in: a brand new campaign is up. Could this go viral with your push? You can RT now or check your dashboard to confirm eligibility. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "📣 Attention collaborators! A new campaign wants your signal boost. Think your followers would engage with it? You’re in control. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🧃 New drop on deck! What’s your gut say—signal boost or skip? You can RT now and check your dashboard later. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "💥 It’s go time! A fresh campaign has launched. Should this one catch fire, or stay quiet? You help decide. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🌀 A new opportunity to make something trend has arrived. Is this campaign worth your influence? Feel it out. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🎯 Time to curate! A campaign just launched—would your audience care? Think before you boost. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "📊 Trendwatch: a new campaign is live. Your repost could push it into the spotlight—or not. That’s up to you. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "👑 New campaign just entered the feed. Your voice gives it power—use it wisely. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🔍 Campaign check: Is this worth your stamp of approval? Your repost shapes what people see. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "💫 A campaign just dropped, but will it rise? Only if you say so. Think it over, then act. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🎲 New campaign, new decision. You can check your dashboard to be sure, or post now if it fits your style. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🚀 Let’s go! A fresh campaign just hit the feed. If it clicks with you, give it that push. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🧠 Think fast! A new campaign wants attention. Will your followers care? You decide what’s worth promoting. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "📬 You’ve got mail: a brand new campaign. Before you repost, ask yourself—would you click this? Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "🎉 Fresh campaign just launched! You don’t need to decide right now—just check it out and see if it fits your vibe. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "💌 A new campaign is up. Is it your style? Would your audience care? Your repost is your recommendation. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",

    "📢 Here we go again! Another campaign joins the mix. You know the drill: scan, decide, share—if it feels right. Pays {rate} DOC per 100 points, between {min} - {max} DOC.\nJoin https://asami.club",
];

const CAMPAIGN_ALERTS_ES: [&str; 20] = [
    "🚨 ¡Nueva campaña en juego! ¿La ves como tendencia? Si crees que tiene potencial, tu retuit puede hacer la diferencia. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "👀 Una nueva campaña acaba de salir. ¿Tu comunidad le haría caso? Evalúa bien antes de amplificar. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🔥 Campaña recién salida del horno. ¿Le das luz verde o no va contigo? Puedes retuitear ahora y revisar si eres elegible después. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "💫 ¡Atención! Hay nueva campaña en circulación. ¿Crees que tus seguidores conectarían con esto? Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🧠 ¿Te convence esta campaña? ¿Vale un retuit? Puedes mirar tu panel o actuar por intuición. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "📣 Nueva campaña disponible. Tú decides si esto merece circulación o no. Puedes revisar tu panel, o simplemente compartir. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🎯 Nueva campaña en la línea. ¿La compartes o la dejas pasar? Recuerda que el pago depende de tu elegibilidad. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "💥 Campaña nueva. ¿Esto le interesa a tu audiencia? Puedes compartir ya o checar tu panel antes. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🌀 ¿Ves potencial en esta campaña? Evalúa antes de dar RT, o lánzate sin miedo y revisa después. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🧃 Nuevo contenido en camino. ¿Merece tu pulgar arriba? Solo tú puedes decidir si esto se vuelve tendencia. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "✨ Campaña disponible. ¿La empujas o pasas? Tu influencia tiene valor, así que elige bien. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "📬 Te llegó una campaña. ¿Coincide con tu estilo o prefieres dejarla pasar? Mira tu panel si dudas. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "👋 Hay campaña nueva. ¿La ves en tu feed? Puedes confirmar en el panel o hacer RT y ver después. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🎲 ¿Te la juegas con esta campaña? Puede que no esté activa para ti, pero igual puedes compartirla. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🔍 Oportunidad de campaña. Tu RT puede hacer que esto llegue lejos. O no. Tú mandas. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🌱 Algo nuevo germina en Asami. ¿Es momento de regarlo con un retuit? Solo si crees que vale. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "💌 Campaña recién salida. ¿Va con tu audiencia? Tu decisión define si esto despega. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "📊 Una campaña más para evaluar. ¿La compartes o no? Revisa tu panel si quieres confirmar antes. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "🎉 ¡Tenemos campaña nueva! ¿Crees que puede destacar en tu timeline? Solo tú tienes esa palanca. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",

    "👑 ¿Este contenido merece tu sello? Revisa tu panel si quieres asegurarte, o simplemente dale RT. Paga {rate} DOC por 100 puntos, entre {min} y {max} DOC.\nÚnete: https://asami.club",
];
