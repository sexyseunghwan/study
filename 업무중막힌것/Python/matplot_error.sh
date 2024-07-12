# ===============================================
# ========== matplotlib 한글 인식 오류 문제 ==========
# ===============================================
## matplotlib 를 사용하다가 한글 인코딩문제로 인해서 폰트를 다운받았는데 적용이 안되는 문제가 발생. (mac, ubunutu 에서 모두 발생)
## 문제해결을 위해서 문제가 생긴 시스템 내에서 python3 를 시작하면 된다.

>>> python3
>>> import matplotlib as mpl
>>> import matplotlib.pyplot as plt
>>> import matplotlib.font_manager as fm


>>> print ('버전: ', mpl.__version__)
버전:  3.5.2
>>> print ('설치 위치: ', mpl.__file__)
설치 위치:  /Users/we/opt/anaconda3/lib/python3.9/site-packages/matplotlib/__init__.py
>>> print ('설정 위치: ', mpl.get_configdir())
설정 위치:  /Users/we/.matplotlib
>>> print ('캐시 위치: ', mpl.get_cachedir())
/Users/we/.matplotlib
>>> print ('설정파일 위치: ', mpl.matplotlib_fname())
/Users/we/opt/anaconda3/lib/python3.9/site-packages/matplotlib/mpl-data/matplotlibrc


## 설정파일 위치로 들어가서 matplotlibrc 파일을 열어서 font.family 쪽 부분 주석을 해제하고 원하는 서체의 이름을 입력하면 된다.
## 그다음 캐시 위치로 이동해서 fontlist-~.json 파일을 지워주고 다시 프로그램 실행하니 해결되었다.



(base)  we@2022080007  ~/Documents/work_code/python_graph_api   main  python3
Python 3.12.4 (main, Jun  6 2024, 18:26:44) [Clang 15.0.0 (clang-1500.3.9.4)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import matplotlib as mpl
>>> import matplotlib.pyplot as plt
>>> import matplotlib.font_manager as fm
>>> print ('버전: ', mpl.__version__)
버전:  3.7.5
>>> print ('설치 위치: ', mpl.__file__)
설치 위치:  /usr/local/lib/python3.12/site-packages/matplotlib/__init__.py
>>> print ('설정 위치: ', mpl.get_configdir())
설정 위치:  /Users/we/.matplotlib
>>> print ('캐시 위치: ', mpl.get_cachedir())
캐시 위치:  /Users/we/.matplotlib
>>> print ('설정파일 위치: ', mpl.matplotlib_fname())
설정파일 위치:  /usr/local/lib/python3.12/site-packages/matplotlib/mpl-data/matplotlibrc
>>> 