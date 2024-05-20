import sys
from queue import Queue



def test():
    queue = Queue()

    queue.put((1,3))

    x,y = queue.get()
    print(x)
    print(y)
    #print(queue.get())
    
    #stack = [(1,2),(3,4),(5,6),(7,8)]

    # while stack:
    #     x, y = stack.pop()
    #     print('x : {}, y : {}'.format(x,y))



test()

