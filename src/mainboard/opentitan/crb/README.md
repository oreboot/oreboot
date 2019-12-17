opentitan README
===============

For more on opentitan search for lowrisc open titan

You need at least the verilator and bitstream. It's a bit clumsy right now: you have to get
build artifacts from azure.

Go here:
[build artifacts](https://dev.azure.com/lowrisc/opentitan/_build?definitionId=5&_a=summary)

Click on any of the builds, and on the top right you should see a link for artifacts. Click on that,
and you will see a choice of 4 items; take the topmost.

It is a gzip tar file; unpack it somewhere. For the verilator, the path will be something like:
```
/path/to/some/place/opentitan-snapshot-20191101-2/hw/top_earlgrey/Vtop_earlgrey_verilator
```

If you do this:
```
PATH=$PATH:/path/to/some/place/opentitan-snapshot-20191101-2/hw/top_earlgrey
```

Running:
```
cargo make -p release run
```
in this directory will run the verilator.

You will see trace output in a file named
```
trace_core_00000000.log
```
