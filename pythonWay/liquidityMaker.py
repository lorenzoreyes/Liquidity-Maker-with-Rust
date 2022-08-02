'''
The goal of this script is to have a python approach to the challenge
and break it down into pieces into rust.
Listen to 5 pairs (configurable), take the bid-ask, gather it into obtain 
the spread and publish it
Once obtained the spread suggest which pair has the lowest spread and suggested
in order to trigger strategies for market maker
'''
from binance import Client, ThreadedWebsocketManager, ThreadedDepthCacheManager
import pandas as pd, yfinance as yahoo, os
import numpy as np, datetime as dt
import matplotlib.pyplot as plt
from pylab import mpl
from controlpanel import *
from ikki import *
mpl.rcParams['font.family'] = 'serif'
plt.style.use('fivethirtyeight')

client = Client(API_KEY,API_SECRET)

pairs = []
length = int(input("How many pairs do you want to track?\t\t"))

for i in range(length):
    pairs.append(input("Provide a pair to track\n\t\t").upper())
    
dcm = ThreadedDepthCacheManager()
# start is required to initialise its internal loop
dcm.start()

def handle_depth_cache(depth_cache):
    symbol = depth_cache.symbol
    # gather the depth 10 tpo bid-asks
    bids, asks = depth_cache.get_bids()[:10],depth_cache.get_asks()[:10]
    # convert them into pandas to get the spread as a weigthed sum quantity*price
    bids, asks = pd.DataFrame(bids,columns=['Quantity','Price']),pd.DataFrame(asks,columns=['Quantity','Price'])
    bids['Spread'] = sum(bids['Quantity'] * bids['Price']) / len(bids)
    asks['Spread'] = sum(asks['Quantity'] * asks['Price']) / len(asks)
    spread = float(bids.Spread.values[0]- asks.Spread.values[0])
    # obtain the timestamp
    time = dt.datetime.fromtimestamp(depth_cache.update_time/1000).strftime('%Y-%m-%d %H:%M:%S.%f')
    # end it up with the orderbook
    orderbook = {'timestamp':time,f'spread-{symbol}':spread}
    orderbook = pd.DataFrame([orderbook.values()],columns=orderbook.keys())
    print(orderbook.tail(1))

for i in range(len(pairs)):
    dcm.start_depth_cache(handle_depth_cache, symbol=pairs[i])

dcm.join()
