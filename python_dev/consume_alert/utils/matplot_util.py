from config import global_logger
from utils.common_util import *
from import_data.common import *


class ConsumeInfoDict:

    def __init__(self, total_cost, start_date, end_date, consume_res_list):
        self.total_cost = total_cost
        self.start_date = start_date
        self.end_date = end_date
        self.consume_res_list = consume_res_list


# 
def parsing_consume_object(total_cost, start_date, end_date, consume_list):

    order_dict = OrderedDict()
    consume_res_list = []
    current_date = start_date

    while current_date <= end_date:
        order_dict[current_date.strftime('%Y-%m-%d')] = 0
        current_date += timedelta(days=1)

    for elem in consume_list:
        
        elem_date = elem.date 
        elem_cost = elem.cost 
        
        # parsing into datetime objects
        dt = datetime.fromisoformat(elem_date.rstrip('Z'))
        # Extract date in year-month-day format
        date_only = dt.date().isoformat()
        
        order_dict[date_only] += elem_cost
        
    total_consume = 0

    for date, total_cost in order_dict.items():
        total_consume += total_cost
        consume_res_list.append(total_consume)

    consume_info = ConsumeInfoDict(total_cost, start_date, end_date, consume_res_list)
    
    return consume_info


# 
def calculate_cosume_res(consume_list, total_cost, start_date, end_date, consume_pre_list, total_pre_cost, pre_start_date, pre_end_date):
    
    consume_info = parsing_consume_object(total_cost, start_date, end_date, consume_list)
    consume_info_pre = parsing_consume_object(total_pre_cost, pre_start_date, pre_end_date, consume_pre_list)
    
    draw_graph_dual(consume_info, consume_info_pre)
    
    #return results


# Formatter Function Definition
def thousands_formatter(x, pos):
    return f'{int(x):,}'


def draw_graph_dual(consume_info_1, consume_info_2):
    
    longer_len = 0

    consume_info_1_len = len(consume_info_1.consume_res_list)
    consume_info_2_len = len(consume_info_2.consume_res_list)

    if consume_info_1_len != consume_info_2_len:
        # Determine the shorter list and the gap in length
        shorter_info = consume_info_1 if consume_info_1_len < consume_info_2_len else consume_info_2
        longer_len = max(consume_info_1_len, consume_info_2_len)
        gap = longer_len - len(shorter_info.consume_res_list)
        
        # Append the last element of the shorter list to itself until lengths match
        last_element = shorter_info.consume_res_list[-1]
        shorter_info.consume_res_list.extend([last_element] * gap)
    else:
        longer_len = consume_info_1_len
    
    x = [i+1 for i in range(longer_len)]



    # if consume_info_1_len > consume_info_2_len:
    #     count_gap = consume_info_1_len - consume_info_2_len 
        
    #     for _ in range(0,count_gap):
    #         consume_info_2.consume_res_list.append(consume_info_2.consume_res_list[consume_info_2_len])

    #     consume_max_len = consume_info_1_len
    # elif consume_info_1_len == consume_info_2_len:
    #     consume_max_len = len(consume_info_1.consume_res_list)
    # else:
    #     count_gap =  consume_info_2_len - consume_info_1_len
        
    #     for _ in range(0,count_gap):
    #         consume_info_1.consume_res_list.append(consume_info_1.consume_res_list[consume_info_1_len])

    #     consume_max_len = consume_info_2_len


    #consume_info_max_len = max(len(consume_info_1.consume_res_list), len(consume_info_1.consume_res_list))
    
    # for i in range(0,consume_info_max_len):
    #     x.append(i)
    

    # 그래프 생성
    plt.figure()
    plt.plot(x, consume_info_1.consume_res_list, marker='o', color='red', label=consume_info_1.total_cost)
    plt.plot(x, consume_info_2.consume_res_list, marker='o', color='black', label=consume_info_2.total_cost)

    # y축 레이블 포맷 설정
    formatter = FuncFormatter(thousands_formatter)
    plt.gca().yaxis.set_major_formatter(formatter)


    plt.title('Consume Plot')
    plt.xlabel('Date')
    plt.ylabel('Consume Cost')  

    plt.legend()
    plt.savefig('./data/img/plot.png')  
    plt.close()  

