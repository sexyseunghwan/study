GET nosql_metric_log-2024.02.05/_search
{
  "query": {
    "bool": {
      "filter": [
        { "term": { "metric_type": "shard_usage" } },
        { "term": { "alert_yn": true } }
      ]
    }
  },
  "sort": [
    { "@timestamp": { "order": "desc" } }
  ]
}



GET nosql_metric_log-2024.02.05/_search
{
  "query": {
    "bool": {
      "filter": [
        { "term": { "metric_type": "shard_usage" } },
        { "term": { "alert_yn": true } }
      ]
    }
  },
  "sort": [
    { "@timestamp": { "order": "desc" } }
  ]
}

