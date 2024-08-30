# ====================================
# ========== 검색쿼리 기본 양식 ==========
# ====================================
PUT /
{
  "settings": {
    "index": {
      "number_of_shards": 3,
      "number_of_replicas": 1
    }
  },
  "mappings": {
    "properties": {
      "name": {
        "type": "text",
        "fields": {
          "keyword": {
            "type": "keyword",
            "ignore_above": 256
          }
        }
      },
      "price": {
        "type": "double"
      },
      "description": {
        "type": "text"
      },
      "manufacturer": {
        "type": "keyword"
      },
      "in_stock": {
        "type": "boolean"
      },
      "tags": {
        "type": "keyword"
      }
    }
  }
}




# ====================================
# ========== 검색쿼리 기본 양식 ==========
# ====================================
GET /consuming_index_prod_nori/_search
{
    "query": {
        "term": {
        "keyword_type": {   ## 특정 필드 즉 where keyword_type = "식사"
            "value": "식사"
        }
        }
    },
  "size" : 100 ## top 쿼리라고 생각하면 됨.
}


# ===========================================================
# ========== 검색쿼리 - 특정 단어를 포함하는 모든 문서를 검색 ==========
# ===========================================================
GET /consuming_index_prod_type_nori/_search
{
  "query": {
    "match": {
      "keyword": "세븐일레븐 잠실"
    }
  }
}



# ====================================================
# ========== 인덱스의 특정 필드의 distinct 값 출력 ==========
# ====================================================
GET /consuming_index_prod_type/_search
{
  "size": 0,  
  "aggs": {
    "unique_keyword_types": {
      "terms": {
        "field": "keyword_type",
        "size": 100  
      }
    }
  }
}


# =============================================
# ========== 인덱스의 where and update ==========
# =============================================
POST /consuming_index_prod_type/_update_by_query
{
  "script": {
    "source": "ctx._source['bias_value'] = 3", ## 해당 부분에 특정 필드값을 넣고 지정해줄 값을 입력한다. -> 처음입력한 값을 토대로 데이터타입이 정해진다.
    "lang": "painless"
  },
  "query": {
    "term": { ## 아래의 정보들이 where 절에 오는 조건이다.
      "keyword_type": {
        "value": "카페"
      }
    }
  }
}



# ==========================================================
# ========== 인덱스 특정 문서의 id를 알 경우에 update 방법 ==========
# ==========================================================
POST /consuming_index_prod_type/_doc/JYduRZABLPyTWz752BYg/_update ## _doc 와 _update 사이 구문에 문서의 id 를 넣어줘야 한다.
{
  "doc": {
    "bias_value": "1" ## 어떤 필드를 어떤 값으로 바꿀것인지 정해줌.
  }
}


# ==========================================================
# ========== 인덱스 특정 문서를 특정 조건으로 정렬시키는 방법 ==========
# ==========================================================
GET /consuming_index_prod_new/_search
{
  
  "sort": [
    { "@timestamp": { "order": "desc" }}
  ]
}




# =============================================================
# ========== 특정 기간안에 특정 필드를 다 더한값을 조회하는 방법 ==========
# =============================================================
GET /consuming_index_prod_new/_search
{
  "size": 10000,
  "query": {
    "range": {
      "@timestamp": {
        "gte": "2024-06-01",
        "lte": "2024-07-02"
      }
    }
  },
  "aggs": {
    "total_prodt_money": {
      "sum": {
        "field": "prodt_money"
      }
    }
  }
}