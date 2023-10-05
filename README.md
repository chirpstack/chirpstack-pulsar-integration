# ChirpStack Pulsar integration

ChirpStack Pulsar integration publishes [ChirpStack integration events](https://www.chirpstack.io/docs/chirpstack/integrations/events.html)
to an [Apache Pulsar](https://pulsar.apache.org/) instance. It reads
integration events by directly subscribing to the Redis event [stream](https://redis.io/docs/data-types/streams/).

## Configuration

```toml
[logging]
  # Log level.
  level="info"

[redis]
  # Server address or addresses.
  #
  # Use rediss:// in case of a TLS secured connection.
  #
  # Example formats:
  #   redis://127.0.0.1:6379
  #   rediss://127.0.0.1:6379
  #   redis://:password@127.0.0.1:6379
  #   redis://username:password@127.0.0.1:6379
  #
  # Set multiple addresses when connecting to a cluster.
  servers=[
    "redis://127.0.0.1/",
  ]

  # Redis Cluster.
  #
  # Set this to true when the provided URLs are pointing to a Redis Cluster
  # instance.
  cluster=false

  # Key prefix.
  #
  # A key prefix can be used to avoid key collisions when multiple deployments
  # are using the same Redis database and it is not possible to separate
  # keys by database index (e.g. when using Redis Cluster, which does not
  # support multiple databases).
  key_prefix=""

  # Consumer and consumer group name.
  #
  # This integration reads the events directly from the Redis event stream.
  #
  # If you are running multiple instances of this integration and you would
  # like to avoid receiving duplicated events, then all instances must share
  # the same consumer-group, each with an unique consumer name. For more
  # information about Redis Streams, see:
  # https://redis.io/docs/data-types/streams/#consumer-groups
  consumer_group="integration_pulsar"

  # Consumer name.
  consumer_name="main"

[pulsar]
  # Pulsar server URL.
  server="pulsar://127.0.0.1:6650"

  # Event topic.
  event_topic="application.{{application_id}}.device.{{dev_eui}}.event.{{event}}"

  # Authentication token (JWT).
  auth_token=""

  # Publish events as JSON instead of Protobuf (binary).
  json=true
```

## Credits

This integration is based on the [Pulsar integration changes](https://github.com/netmoregroup/chirpstack/tree/pulsar)
by [Spindel](https://github.com/Spindel).

## License

The ChirpStack Pulsar integration is distributed under the MIT license. See
also [LICENSE](https://github.com/chirpstack/chirpstack-pulsar-integration/blob/master/LICENSE).
