from config import global_logger
from utils.common_util import *
from import_data.common import *


# class DateCosume:

#     def __init__(self, date, consume):
#         self.date = date
#         self.consume = consume

#     def consume_sum(self, cost):
#         self.consume += cost


def calculate_cosume_res(consume_list):
    
    # 결과를 저장할 두 리스트
    dates_only = []
    dates_with_values = []

    for elem in consume_list:

        elem_date = elem.date 
        elem_cost = elem.cost 
        #print(elem_date, elem_cost)
    
        # datetime 객체로 파싱
        dt = datetime.fromisoformat(elem_date.rstrip('Z'))

        # 년-월-일 형식으로 날짜 추출
        date_only = dt.date().isoformat()

        if date_only in dates_only:
            date_idx = dates_only.index(date_only)
            dates_with_values[date_idx] += elem_cost
        else:    
            dates_only.append(date_only)
            dates_with_values.append(elem_cost)

    for i in range(0, len(dates_only)):
        print(dates_only[i], dates_with_values[i])


#def draw_graph(x_list, y_list):
    