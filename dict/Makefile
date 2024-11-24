DICT_SIZE = 10000

all: en ru

en:
	tail -n +2 ./original/en_333k.csv | head -n ${DICT_SIZE} | cut -d ',' -f 1 > en.dict

ru:
	head -n ${DICT_SIZE} ./original/ru_100k.txt > ru.dict
