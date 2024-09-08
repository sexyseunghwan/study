# ====================================================================
# ========== 인덱스의 현재 설정 확인: 먼저 현재 인덱스의 설정을 확인한다. ==========
# ====================================================================
GET /my_index/_settings



# ======================================
# ========== 레플리카 샤드 수 변경 ==========
# ======================================
PUT /my_index/_settings
{
  "index": {
    "number_of_replicas": 3  // 변경하고자 하는 레플리카 수
  }
}



# ====================================================================
# ========== 애널라이저 적용 & 인덱스 생성 & 동의어, 사전 추가하는 방법. ==========
# ====================================================================
## user_dictionary_rules => 이쪽부분에 사용자가 설정할 단어를 넣는것이고
## synonyms => 이쪽 부분에 동의어를 넣으면 된다.
PUT /my_index_with_nori
{
  "settings": {
    "index": {
      "analysis": {
        "tokenizer": {
          "nori_user_dict": {
            "type": "nori_tokenizer",
            "decompound_mode": "mixed",
            "user_dictionary_rules": [
              "스타벅스",
              "삼성전자",
              "현대자동차"
            ]
          }
        },
        "filter": {
          "my_synonyms": {
            "type": "synonym_graph",
            "synonyms": [
              "스타벅스, 스타벅스커피 => 스타벅스",
              "삼성, 삼성그룹 => 삼성",
              "현대, 현대차 => 현대자동차"
            ]
          },
          "nori_pos_filter": {
            "type": "nori_part_of_speech",
            "stoptags": ["E", "IC", "J", "MAG", "MM", "NA", "NR", "SC", "SE", "SF", "SH", "SL", "SN", "SP", "SSC", "SSO", "SY", "UNA", "UNKNOWN", "VA", "VCN", "VCP", "VV", "VX", "XPN", "XR", "XSA", "XSN", "XSV"]
          }
        },
        "analyzer": {
          "custom_nori_analyzer": {
            "type": "custom",
            "tokenizer": "nori_user_dict",
            "filter": [
              "nori_pos_filter",
              "lowercase",
              "my_synonyms"
            ]
          }
        }
      }
    }
  },
  "mappings": {
    "properties": {
      "description": {
        "type": "text",
        "analyzer": "custom_nori_analyzer"
      }
    }
  }
}

## 스키마 구조까지 설정해주고 싶은 경우.
PUT /consuming_index_prod_nori
{
  "settings": {
    "index": {
      "analysis": {
        "tokenizer": {
          "nori_user_dict": {
            "type": "nori_tokenizer",
            "decompound_mode": "mixed",
            "user_dictionary_rules": [
              "스타벅스"
            ]
          }
        },
        "filter": {
          "my_synonyms": {
            "type": "synonym_graph",
            "synonyms": [
              "스타벅스, 스타벅스커피 => 스타벅스"
            ]
          },
          "nori_pos_filter": {
            "type": "nori_part_of_speech",
            "stoptags": ["E", "IC", "J", "MAG", "MM", "NA", "NR", "SC", "SE", "SF", "SH", "SL", "SN", "SP", "SSC", "SSO", "SY", "UNA", "UNKNOWN", "VA", "VCN", "VCP", "VV", "VX", "XPN", "XR", "XSA", "XSN", "XSV"]
          }
        },
        "analyzer": {
          "custom_nori_analyzer": {
            "type": "custom",
            "tokenizer": "nori_user_dict",
            "filter": [
              "nori_pos_filter",
              "lowercase",
              "my_synonyms"
            ]
          }
        }
      }
    }
  },
  "mappings": {
    "properties": {
      "name": {
        "type": "text",
        "analyzer": "custom_nori_analyzer"
      },
      "description": {
        "type": "text",
        "analyzer": "custom_nori_analyzer"
      },
      "date": {
        "type": "date"
      },
      "price": {
        "type": "double"
      },
      "tags": {
        "type": "keyword"
      },
      "location": {
        "type": "geo_point"
      }
    }
  }
}





# =============================
# ========== reindex ==========
# =============================
# 기존의 인덱스의 모든 내용을 새로운 인덱스로 옮길 경우에 사용한다.
POST _reindex
{
  "source": {
    "index": "old_index"
  },
  "dest": {
    "index": "new_index"
  }
}


# ============================================
# ========== 애널라이저 결과 확인하는 방법 ==========
# ============================================
GET /consuming_index_prod_nori/_analyze
{
  "text": "스타벅스 강남",
  "analyzer": "my_analyzer"
}

