# woteconv
command-line tool that takes a commonevent file and prints a text conversion to stdout

the output isn't super readable (it's similar to the editor's built-in text conversion), but it may be useful for git textconv

## usage
```
woteconv.exe <file>
```
file name must contain `CommonEvent.dat` or `.common` to be recognized

## example output
```

>> dheap
    (103,)("functions to manage dynamic allocation of statically sized cdb data",)
    (103,)("for a target cdb type that is meant to have dynamic allocation, one value must be reserved at the 0th slot and initialized to -1",)
    (0,)()


returns data id of the newly allocated data (val 0 must be reserved and init to -1); ret -1 if failed
dheap::alloc
    (250,1600000,0,0,4096,1600010,)("","","","",)
    (111,1,1600010,10000,1,)()
    (401,1,)()
        (121,1600010,-1,0,0,)()
        (172,)()
        (0,)()
    (499,)()
    (111,1,1600010,-1,2,)()
    (401,1,)()
        (121,1600010,1,0,0,)()
        (0,)()
    (499,)()
    (250,1600000,1600010,0,4096,1600011,)("","","","",)
    (111,1,1600011,-1,2,)()
    (401,1,)()
        (121,1600011,1600010,1,0,)()
        (0,)()
    (499,)()
    (250,1600000,0,0,0,1600011,)("","","","",)
    (0,)()


deallocates the idx at the given cdb
dheap::free
    (250,1600000,1600001,-2,0,0,)("","","","",)
    (250,1600000,0,0,4096,1600010,)("","","","",)
    (250,1600000,1600001,0,0,1600010,)("","","","",)
    (250,1600000,0,0,0,1600001,)("","","","",)
    (0,)()


resets the given cdb heap
dheap::clear
    (252,1600000,0,0,4096,10000,)("","","","",)
    (0,)()



```
