# The Bench
## A NAVCON Testing Platform

"The Bench" allows you to test your NAVCON algorithm extensively!

It does this by allowing you to enter test data into a text file, 
and then simulate the SS and MDPS using the that data.
### Text File Syntax

For now, only SS input data has been implemented. So the syntax for
a Test Protocol Text File (TPTF) is as follows:
<br />
-> sensor colours<br />
-> incidence<br />
-> sensor colours<br />
-> incidence<br />
-> sensor colours<br />
-> incidence<br />
-> ......<br />

where sensor colours is a _5_ character _String_, where the colours 
are represented at follows:<br />
    - White: 'W'<br />
    - Red: 'R'<br />
    - Green: 'G'<br />
    - Blue: 'B'<br />
    - Black: 'N'<br />

and incidence is an _8-bit unsigned integer_ (0<incidence<255)

example:
<br />
WWWWW<br />
0<br />
GWWWW<br />
5<br />
GGWWW<br />
5<br />
WWWWW<br />
5<br />
WWWWW<br />
5<br />
BWWWW<br />
45<br />
...

There is no limit on the number of lines in a TPTF, The Bench 
will run all of them!
