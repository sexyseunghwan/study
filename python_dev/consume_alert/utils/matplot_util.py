from config import global_logger
from utils.common_util import *
from import_data.common import *


def calculate_cosume_res(consume_list):
    
    # 결과를 저장할 두 리스트
    results = OrderedDict()

    for elem in consume_list:

        elem_date = elem.date 
        elem_cost = elem.cost 
        
        # datetime 객체로 파싱
        dt = datetime.fromisoformat(elem_date.rstrip('Z'))

        # 년-월-일 형식으로 날짜 추출
        date_only = dt.date().isoformat()

        if date_only in results:
            results[date_only] += elem_cost
        else:
            results[date_only] = elem_cost

    for date, total_cost in results.items():
        print(date, total_cost)


#def draw_graph(x_list, y_list):
    