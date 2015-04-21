st
=====

st is a simple command line utility for computing basics statistics on column
oriented plain text data files. Currently st evaluates the following quantities:

* mean
* standard deviation
* median
* min
* max

Please note that the computation of the median currently requires st to store the data
internally and thus consumes memory on the order of the data set itself.
