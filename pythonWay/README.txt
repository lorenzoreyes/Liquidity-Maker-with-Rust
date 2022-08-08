This folder contains my python approach to the challenge

run
pip install -r requirements.txt


(1) liquidityMaker.py: draft-approach in python in order to 
breakdown into pieces the needed parts of steps to achieve
the challenge.

(2) owlyStop10.py: personal code to perform a SMA trend following
strategy with 10% stop loss. 

(3) controlpanel.py: setup of some endpoints of binance account.

# there is an api account but it is educational, no worries about security.
(4) marketResearch/{bcra.py,curvaRofex22.py}: this folder contains
the market research to generate report of Argentina Economy related
to: 
bcra.py Central Bank of Argentina download && serialized balance sheet
curvaRofex22.py downloads futures over last year and monitor it evolution
rate implied, all with intraday data
to the forex (ARS/USD dollar peso futures), central bank balance sheet
it was a challenge on how to preserve the whole data without being tempted 
to calculate a daily average, because it wouldnt describe the true image
