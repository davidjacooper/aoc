#!/usr/bin/R -s -f

# row.of.interest <- 10
row.of.interest <- 2000000

con = file("stdin")
inputText <- readLines(con)
close(con)
coords <- t(sapply(regmatches(inputText, gregexpr("-?\\d+", inputText)), as.numeric))

sensor.x <- coords[,1]
sensor.y <- coords[,2]
beacon.x <- coords[,3]
beacon.y <- coords[,4]

sensor.range <- abs(sensor.x - beacon.x) + abs(sensor.y - beacon.y)

# 'spread' is the number of units, horizontally, either side of sensor.x, that the sensor should be
# able to see on the row-of-interest.
spread <- sensor.range - abs(sensor.y - row.of.interest)

# Negative spread values mean the sensor is out of range.
spread[spread < 0] <- NA

# Calculate the horizontal ranges that the sensors can see across the row of interest.
range.x.min <- sensor.x - spread
range.x.max <- sensor.x + spread

print(cbind(sensor.x, sensor.y, beacon.x, beacon.y, sensor.range, spread, range.x.min, range.x.max))


# Make a vector for the row of interest, big enough to span all the horizontal ranges seen by all
# sensors. (Note that the conceptual X coordinates may be negative, whereas the vector is indexed
# from 1, so 'lowest.x.min' must be used to convert indexes.)
lowest.x.min <- min(range.x.min, na.rm = TRUE)
highest.x.max <- max(range.x.max, na.rm = TRUE)

# Fill out the row with TRUE, for positions seen by sensors.
full.row <- rep(FALSE, highest.x.max - lowest.x.min + 1)
for(i in 1:length(sensor.x))
{
    if(!is.na(spread[i]))
    {
        full.row[range.x.min[i]:range.x.max[i] - lowest.x.min + 1] <- TRUE
    }
}

# Delete any squares containing actual known beacons.
full.row[beacon.x[beacon.y == row.of.interest] - lowest.x.min + 1] <- FALSE

print(sum(full.row))
