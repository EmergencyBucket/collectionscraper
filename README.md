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