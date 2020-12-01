import pandas as pd
from tqdm import tqdm

def readFile(fileName = './input1.txt'):
    dataList = []
    with open(fileName) as inputFile:
        for line in inputFile.readlines():
            dataList.append( int(line) )
    return dataList

target = 2020
dataList = readFile()
listLength = len( dataList )

for i in tqdm( range(listLength) ):

    for j in range(i+1, listLength):

        for k in range(j+1, listLength):

            if dataList[i] + dataList[j] + dataList[k] == target:
                print( dataList[i] * dataList[j] * dataList[k] )
                print( dataList[i], dataList[j], dataList[k] )
                exit(0)
                
print("Didn't find any")