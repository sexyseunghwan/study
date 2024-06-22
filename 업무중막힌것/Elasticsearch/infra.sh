# 인덱스의 현재 설정 확인: 먼저 현재 인덱스의 설정을 확인한다.
GET /my_index/_settings


# 레플리카 샤드 수 변경
PUT /my_index/_settings
{
  "index": {
    "number_of_replicas": 3  // 변경하고자 하는 레플리카 수
  }
}