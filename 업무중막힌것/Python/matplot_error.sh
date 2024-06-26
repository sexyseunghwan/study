## matplotlib 를 사용하다가 한글 인코딩문제로 인해서 폰트를 다운받았는데 적용이 안되는 문제가 발생.

import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.font_manager as fm


>>> print ('버전: ', mpl.__version__)
버전:  3.5.2

>>> print ('설치 위치: ', mpl.__file__)
설치 위치:  /Users/we/opt/anaconda3/lib/python3.9/site-packages/matplotlib/__init__.py

>>> print ('설정 위치: ', mpl.get_configdir())
설정 위치:  /Users/we/.matplotlib

>>> print ('캐시 위치: ', mpl.get_cachedir())
/Users/we/.matplotlib

>>> print ('설정파일 위치: ', mpl.matplotlib_fname())

## /Users/we/.matplotlib 로 이동해서 fontlist-~.json 파일을 지워주고 다시 프로그램 실행하니 잘 됨.


/usr/share/fonts/truetype/nanum


print (': ', mpl.get_cachedir())

/root/.cache/matplotlib
/root/.config/matplotlib

print (': ', mpl.get_configdir())

>>> print (': ', mpl.matplotlib_fname())

/usr/local/lib/python3.8/dist-packages/matplotlib/mpl-data/matplotlibrc


/Users/we/opt/anaconda3/lib/python3.9/site-packages/matplotlib/mpl-data/matplotlibrc


BM_DoHyeo


/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf


font_prop = font_manager.FontProperties(fname="./data/font/BMDOHYEON_ttf.ttf").get_name()
font_prop = font_manager.FontProperties(fname="/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").get_name()

font_prop = font_manager.FontProperties(fname="/usr/share/fonts/truetype/bmd.ttf").get_name()