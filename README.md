# Emblems Report Collection Scraper
> A fast Rust-based Destiny 2 profile collections scraper.

## Features
 - Random IPv6 binding to bypass per IP ratelimit
 - Hybrid RabbitMQ and Clickhouse user id ingestion
 - ~30,000 profiles scraped per hour. (Does not degrate with number of running instances)

## Requirements
 - A server with an IPv6 subnet
 - A Clickhouse database
 - RabbitMQ stream with new ids (ours is done with an external PCGR scraper)

## Setup
1. Configure your clickhouse database with two tables
``users_v3``:
```sql
DESCRIBE TABLE users_v3

┌─name───────────┬─type──────────┬─default_type─┬─default_expression─┬─comment─┬─codec_expression─┬─ttl_expression─┐
│ timestamp      │ Int64         │              │                    │         │                  │                │
│ membershipId   │ Int64         │              │                    │         │                  │                │
│ membershipType │ Int8          │              │                    │         │                  │                │
│ bungieName     │ String        │              │                    │         │                  │                │
│ lastPlayed     │ Int64         │              │                    │         │                  │                │
│ profileData    │ String        │              │                    │         │                  │                │
│ collections    │ Array(UInt32) │              │                    │         │                  │                │
│ emblems        │ Array(UInt32) │              │                    │         │                  │                │
└────────────────┴───────────────┴──────────────┴────────────────────┴─────────┴──────────────────┴────────────────┘
```

``old_emblem_data``:

```sql
DESCRIBE TABLE old_emblem_data

┌─name───────┬─type──────────┬─default_type─┬─default_expression────────────────────────────────────────────────────────────────────────────────────────────┬─comment─┬─codec_expression─┬─ttl_expression─┐
│ user_id    │ UInt64        │              │                                                                                                               │         │                  │                │
│ platform   │ Int8          │              │                                                                                                               │         │                  │                │
│ source_ip  │ String        │              │                                                                                                               │         │                  │                │
│ emblems    │ Array(UInt32) │ DEFAULT      │ []                                                                                                            │         │                  │                │
│ rankrarity │ UInt64        │ DEFAULT      │ arraySum(arrayResize(arraySort(arrayMap(x -> dictGet('default.rarity_dict', 'user_count', x), emblems)), 10)) │         │                  │                │
│ user_ver   │ DateTime      │ DEFAULT      │ now()                                                                                                         │         │                  │                │
│ networth   │ UInt64        │ DEFAULT      │ arraySum(arrayMap(x -> dictGetOrDefault('default.prices', 'price', x, 0), emblems))                           │         │                  │                │
└────────────┴───────────────┴──────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────┴─────────┴──────────────────┴────────────────┘
```

2. Configure a RabbitMQ stream to output arrays of minified user ids (user_id-4611686018000000000)
3. Set the IPv6 subnet. (More info: https://github.com/zu1k/http-proxy-ipv6-pool)  
``ip a``  
``ip route add local your_ipv6_subnet dev interface_name``  
``sysctl net.ipv6.ip_nonlocal_bind=1``
4. Run the collection scraper

## Future Development
Currently the scraper is limited by Clickhouse ingestion using a lot of CPU resources. Batching larger chunks may solve this issue.