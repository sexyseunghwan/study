from config import global_logger
from utils.common_util import *
from import_data.common import *


def calculate_cosume_res(consume_list):
    
    # Ordered Dictionary to save results
    results = OrderedDict()
    
    for elem in consume_list:

        elem_date = elem.date 
        elem_cost = elem.cost 
        
        # parsing into datetime objects
        dt = datetime.fromisoformat(elem_date.rstrip('Z'))
        
        # Extract date in year-month-day format
        date_only = dt.date().isoformat()

        if date_only in results:
            results[date_only] += elem_cost
        else:
            results[date_only] = elem_cost

    for date, total_cost in results.items():
        print(date, total_cost)

    draw_graph(results)

    #return results


# Formatter Function Definition
def thousands_formatter(x, pos):
    return f'{int(x):,}'


def draw_graph(order_dict):
    
    x = []
    y = []
    
    for date, total_cost in order_dict.items():
        x.append(date)
        y.append(total_cost)

    # 그래프 생성
    plt.figure()
    plt.plot(x, y, marker='o', color='red', label='This')

    # y축 레이블 포맷 설정
    formatter = FuncFormatter(thousands_formatter)
    plt.gca().yaxis.set_major_formatter(formatter)


    plt.title('Consume Plot')
    plt.xlabel('Date')
    plt.ylabel('Consume Cost')  

    plt.legend()
    plt.savefig('./data/img/plot.png')  
    plt.close()  

