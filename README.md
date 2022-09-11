# The Bench
## A NAVCON Testing Platform

"The Bench" allows you to test your NAVCON algorithm extensively!

It does this by allowing you to enter test data into a text file, 
and then simulate the SS and MDPS using the that data.
### Text File Syntax

For now, only SS input data has been implemented. So the syntax for
a Test Protocol Text File (TPTF) is as follows:

-> sensor colours
-> incidence
-> sensor colours
-> incidence
-> sensor colours
-> incidence
-> ......

where sensor colours is a _5_ character _String_, where the colours 
are represented at follows:
    - White: 'W'
    - Red: 'R'
    - Green: 'G'
    - Blue: 'B'
    - Black: 'N'

and incidence is an _8-bit unsigned integer_ (0<incidence<255)

example:

WWWWW
0
GWWWW
5
GGWWW
5
WWWWW
5
WWWWW
5
BWWWW
45
...

There is no limit on the number of lines in a TPTF, The Bench 
will run all of them!