#!/usr/bin/R -s -f

area.x.min = 0
area.y.min = 0
area.x.max = 4000000
area.y.max = 4000000

# -- UNCOMMENT these lines when using 'simple_input' rather than 'input'
# area.x.min = 0
# area.y.min = 0
# area.x.max = 20
# area.y.max = 20

con <- file("stdin")
inputText <- readLines(con)
close(con)
coords <- t(sapply(regmatches(inputText, gregexpr("-?\\d+", inputText)), as.numeric))

sensor.x <- coords[,1]
sensor.y <- coords[,2]
beacon.x <- coords[,3]
beacon.y <- coords[,4]
sensor.range <- abs(sensor.x - beacon.x) + abs(sensor.y - beacon.y)

for(y in area.y.min:area.y.max)
{
    print(y)
    
    # 'spread' is the number of units, horizontally, either side of sensor.x, that the sensor should be
    # able to see on the row-of-interest.
    spread <- sensor.range - abs(sensor.y - y)
    
    # Calculate the horizontal ranges that the sensors can see across the row of interest.
    range.x.min <- sensor.x - spread
    range.x.max <- sensor.x + spread
    
    x <- area.x.min
    for(i in order(range.x.min, range.x.max))
    {
        rmin <- range.x.min[i]
        rmax <- range.x.max[i]
        if(rmin <= rmax)
        {
            if(x < rmin)
            {
                break
            }
            x <- max(x, rmax + 1)
        }
    }
    
    if(x <= area.x.max)
    {
        print(c(x, y))
        print(format(x * 4000000 + y, scientific = FALSE))
        break
    }
}
