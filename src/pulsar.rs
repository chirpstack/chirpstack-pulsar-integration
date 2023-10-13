use anyhow::Result;
use async_trait::async_trait;
use handlebars::Handlebars;
use pulsar::Pulsar;
use serde::Serialize;
use tracing::{info, trace};

use crate::config;
use chirpstack_api::{integration as integration_pb, prost::Message};
use chirpstack_integration::IntegrationTrait;

#[derive(Serialize)]
struct EventTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub event: String,
}
pub struct Integration<'a> {
    client: Pulsar<pulsar::executor::TokioExecutor>,
    templates: Handlebars<'a>,
    json: bool,
}

impl<'a> Integration<'a> {
    pub async fn new(conf: config::Pulsar) -> Result<Integration<'a>> {
        info!("Initializing Pulsar integration");

        // topic template
        let mut templates = Handlebars::new();
        templates.register_escape_fn(handlebars::no_escape);
        templates.register_template_string("event_topic", &conf.event_topic)?;

        let mut client_builder =
            Pulsar::builder(conf.server.clone(), pulsar::executor::TokioExecutor);

        // JWT authentication
        if !conf.auth_token.is_empty() {
            client_builder = client_builder.with_auth(pulsar::Authentication {
                name: "token".into(),
                data: conf.auth_token.clone().into_bytes(),
            });
        }

        let client = client_builder.build().await?;

        Ok(Integration {
            client,
            templates,
            json: conf.json,
        })
    }

    async fn publish_event(&self, topic: &str, payload: Vec<u8>) -> Result<()> {
        info!(topic = %topic, "Publishing event");

        let msg = pulsar::producer::Message {
            payload,
            ..Default::default()
        };

        // Rather than keeping track of producers per-topic, we use the built-in "lazy" option to
        // do so. Less control of schema and other producer options, but simpler implementation.
        let acked = self.client.send(topic, msg).await?;

        // Ack waiting is not mandatory, and can take an arbitrary amount of time, as there may be
        // batching and more happening.
        // In 2022 context, it is okay as events spawn in their own tasks and don't block other
        // progress, however, if that changes, this may require some attention
        trace!(topic = %topic, "Waiting for ack");
        acked.await?;

        Ok(())
    }

    fn get_event_topic(&self, application_id: &str, dev_eui: &str, event: &str) -> Result<String> {
        let topic = self.templates.render(
            "event_topic",
            &EventTopicContext {
                application_id: application_id.to_string(),
                dev_eui: dev_eui.to_string(),
                event: event.to_string(),
            },
        )?;
        Ok(topic)
    }
}

#[async_trait]
impl<'a> IntegrationTrait for Integration<'a> {
    async fn uplink_event(&self, pl: &integration_pb::UplinkEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "up")?;

        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };
        self.publish_event(&topic, payload).await
    }

    async fn join_event(&self, pl: &integration_pb::JoinEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "join")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn ack_event(&self, pl: &integration_pb::AckEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "ack")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn txack_event(&self, pl: &integration_pb::TxAckEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "txack")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn log_event(&self, pl: &integration_pb::LogEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "log")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn status_event(&self, pl: &integration_pb::StatusEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "status")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn location_event(&self, pl: &integration_pb::LocationEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic =
            self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "location")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }

    async fn integration_event(&self, pl: &integration_pb::IntegrationEvent) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic =
            self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "integration")?;
        let payload = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, payload).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::TryStreamExt;
    use pulsar::message::proto::command_subscribe::SubType;
    use regex::Regex;
    use std::env;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_integration() {
        let pulsar_url =
            env::var("TEST_PULSAR_URL").unwrap_or("pulsar://127.0.0.1:6650".to_string());

        let config = config::Pulsar {
            server: pulsar_url.clone(),
            ..Default::default()
        };
        let integration = Integration::new(config).await.unwrap();

        let topic_re = Regex::new(r".*application\..*").unwrap();
        let topic =
            "application.00000000-0000-0000-0000-000000000000.device.0102030405060708.event.up";

        let client = Pulsar::builder(pulsar_url, pulsar::executor::TokioExecutor)
            .build()
            .await
            .unwrap();

        let mut consumer: pulsar::Consumer<Vec<u8>, _> = client
            .consumer()
            .with_consumer_name("test_consumer")
            // Regexp-only topics only work if the pulsar server HAS the topics first.
            // So for a test-case where the topic doesn't exist until _after_ the first publish,
            // we may not see it.
            // This means that the first time tests run, they fail, but succeed afterwards, which
            // is bad.
            .with_topic(topic)
            .with_topic_regex(topic_re)
            .with_subscription("test_subscription")
            .with_subscription_type(SubType::Exclusive)
            .build()
            .await
            .expect("Failed to create consumer");

        // Check that we have a connection before testing
        consumer
            .check_connection()
            .await
            .expect("Consumer connection is not healthy");

        let pl = integration_pb::UplinkEvent {
            device_info: Some(integration_pb::DeviceInfo {
                application_id: "00000000-0000-0000-0000-000000000000".to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let pl_b = pl.encode_to_vec();

        // Spawn a listener before we send messages.
        let handle = tokio::spawn(async move {
            while let Some(msg) = consumer.try_next().await.unwrap() {
                assert_eq!(
                    "persistent://public/default/application.00000000-0000-0000-0000-000000000000.device.0102030405060708.event.up",
                    msg.topic
                );
                assert_eq!(pl_b, msg.payload.data);
                consumer.ack(&msg).await.expect("Failed to ack");
                break;
            }
            consumer
                .close()
                .await
                .expect("Failed to unsubscribe consumer");
        });

        integration.uplink_event(&pl).await.unwrap();

        let _ = timeout(Duration::from_secs(1), handle).await.unwrap();
    }
}
