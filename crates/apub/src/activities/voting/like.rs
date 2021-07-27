use crate::activities::{
  verify_activity,
  verify_person_in_community,
  voting::receive_like_or_dislike,
};
use activitystreams::activity::kind::LikeType;
use lemmy_apub_lib::{ActivityCommonFields, ActivityHandler, PublicUrl};
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikePostOrComment {
  to: PublicUrl,
  pub(in crate::activities::voting) object: Url,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: LikeType,
  #[serde(flatten)]
  common: ActivityCommonFields,
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for LikePostOrComment {
  async fn verify(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    verify_activity(self.common())?;
    verify_person_in_community(&self.common.actor, &self.cc[0], context, request_counter).await?;
    Ok(())
  }

  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    receive_like_or_dislike(
      1,
      &self.common.actor,
      &self.object,
      context,
      request_counter,
    )
    .await
  }

  fn common(&self) -> &ActivityCommonFields {
    &self.common
  }
}
