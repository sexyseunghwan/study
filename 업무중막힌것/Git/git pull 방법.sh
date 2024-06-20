# 1. Git 설치하기 (아직 설치하지 않았다면)
sudo apt update
sudo apt install git

# 2. Pull 실행: 현재 브랜치에 원격 리포지토리의 최신 변경사항을 가져오기 위해 git pull 명령을 실행한다. 
#              이 명령은 원격 리포지토리의 변경사항을 현재 로컬 브랜치로 병합한다.
git pull origin main




######## 무조건 마스터의 커밋정보만 동기화 하려고 한다면 ########
# 1. 현재 변경사항을 스태시하거나 버리기
git reset --hard

# 2. main 브랜치에서 pull 하기
git pull origin main
