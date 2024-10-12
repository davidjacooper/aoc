using System;
using System.Collections;
using System.Collections.Generic;

class Day17_Part2
{
    static readonly int[][] rocks =
    {
        // ####
        new int[] {0b0000000, 0b0000000, 0b0000000, 0b0011110},
        
        // .#.
        // ###
        // .#.
        new int[] {0b0000000, 0b0001000, 0b0011100, 0b0001000},
        
        // ..#
        // ..#
        // ###
        new int[] {0b0000000, 0b0000100, 0b0000100, 0b0011100},
                                           
        // #
        // #
        // #
        // #
        new int[] {0b0010000, 0b0010000, 0b0010000, 0b0010000},
                                                        
        // ##
        // ##
        new int[] {0b0000000, 0b0000000, 0b0011000, 0b0011000}
    };
    

    static void Main(string[] args)
    {
        var directions = Console.ReadLine();
        int rockType = 0;
        int jetIndex = 0;
        
        var rockPile = new ArrayList();
        int[] maxRockHeight = new int[]{0,0,0,0,0,0,0};
        
        // Periodicity analysis (for predicting the total eventual height)
        var lastSeen = new Dictionary<long,long>();
        long period = 0;
        long heightSoFar = 0;
        long periodicIncrease = 0;
        
        // The upper limit just needs to be high enough for periodicity analysis and prediction to
        // gather enough data.
        for(long n = 0; n < 100000; n++)
        {
            // Spawn new rock
            long rock = ((long)rocks[rockType][0] << 32) +
                        ((long)rocks[rockType][1] << 24) +
                        ((long)rocks[rockType][2] << 16) +
                        ((long)rocks[rockType][3] << 8);
            
            rockType++;
            if(rockType >= rocks.Length)
            {
                // Wrap around the rock type
                rockType = 0;
            }
            
            // We're creating a 40-bit (8x5) bitmap, which (if attanged in rows of 8 bits)
            // looks like this:
            //
            // 76543210
            // .nnnnnnn 32
            // .nnnnnnn 24
            // .nnnnnnn 16
            // .nnnnnnn  8
            // ........  0
            //
            // The shape of the rock fits within the 7x4 'n' region. The extra line below
            // represents the next row below, and the extra column to the left represents the
            // walls (simultaneously both the left and right walls, since the structure wraps
            // around).
            
            var height = rockPile.Count + 2;
            // The spec says '3', but the bitmap extends down an extra row, and 'height' says where
            // _that_ row is.
            
            while(true)
            {
                if(height < -5)
                {
                    Console.WriteLine("  Oops");
                    break;
                }
                
                // 'Obstacles' is another equal-sized bitmap, representing the walls and relevant
                // parts of piled-up rocks.
                long obstacles = (1L << 39) + (1L << 31) + (1L << 23) + (1L << 15) + (1L << 7); // The walls
                
                for(int i = height; i <= (height + 4) && i < rockPile.Count; i++)
                {
                    // Figure out what fallen rocks already exist at level i (which can be negative).
                    long existingRocks = (i < 0) ? 0b1111111 : (int)rockPile[i];
                    
                    // Place these into the appropriate position in the bitmap:
                    // * "i - height" is the bitmap row, and 8 is the size in bits of each row.
                    obstacles |= existingRocks << ((i - height) * 8);
                }
                
                // The left/right rock motion corresponds to the shift-left/right operations.
                long movedRock = (directions[jetIndex] == '<') ? (rock << 1) : (rock >> 1);
                
                jetIndex++;
                if(jetIndex >= directions.Length)
                {
                    // Wrap around the jet direction pattern.
                    jetIndex = 0;
                }
                
                if((movedRock & obstacles) == 0)
                {
                    // If the bitmaps have any bits in common, it means the rock hit something.
                    // It can only move if it doesn't.
                    rock = movedRock;
                }
                
                // See if the rock is able to move down (>> 8).
                if(((rock >> 8) & obstacles) != 0)
                {
                    break;
                }
                height--;
            }
            
            // Now the rock should be 'pasted' into list.
            for(int i = height + 1; i <= (height + 4); i++)
            {
                int rockRow = (int) ((rock >> ((i - height) * 8)) & 0b1111111);
                
                if(rockRow == 0)
                {
                    break;
                    // Assume that a zero row means we've reached the top of the rock. We don't
                    // want to add zero-rows to the rock pile.
                }
                
                // Keep track of the highest rock-unit in each column. (Overrides previous heights,
                // and also overrides previous iterations of this loop as i increases.)
                for(int j = 6; j >= 0; j--)
                {
                    if((rockRow & (1 << j)) != 0)
                    {
                        maxRockHeight[j] = i;
                    }
                }
                
                // Either add to the rock pile, or superimpose on an existing part of it.
                if(i >= rockPile.Count)
                {
                    rockPile.Add(rockRow);
                    
                    // Used to make the height prediction. (rockPile.Count is no help, since we
                    // truncate it every so often.)
                    periodicIncrease++;
                    heightSoFar++;
                }
                else
                {
                    rockPile[i] = (int)rockPile[i] | rockRow;
                }
            }
            
            // Find the lowest of the maximum rock heights, on the grounds that we can delete parts
            // of the rock pile below this height without affecting anything.
            int minMaxRockHeight = Int32.MaxValue;
            for(int j = 6; j >= 0; j--)
            {
                if(minMaxRockHeight > maxRockHeight[j])
                {
                    minMaxRockHeight = maxRockHeight[j];
                }
            }
            
            if(minMaxRockHeight > 1000)
            {
                rockPile.RemoveRange(0, minMaxRockHeight);
                for(int j = 6; j >= 0; j--)
                {
                    maxRockHeight[j] -= minMaxRockHeight;
                }
            }
            
            // Analyse periodicity and predict total eventual height.
            
            // First, get the bitmap from the top of the rock pile
            long bitmap = 0L;
            int count = rockPile.Count;
            for(int i = count - 8; i < count; i++)
            {
                long bits = (i < 0) ? 0 : (int)rockPile[i];
                bitmap |= (bits << (i - count + 8));
            }
            
            // Has this bitmap occurred before?
            long prevN;
            if(lastSeen.TryGetValue(bitmap, out prevN))
            {
                var interval = n - prevN;
                
                // Deduce the period of the height increases, by searching for the longest
                // repetition interval. Shorter intervals will be false positives, but there cannot
                // be any intervals longer than the global period.
                if(period < interval)
                {
                    period = interval;
                }
            }
            
            // Record the time of this bitmap, so we can calculate the interval when we see it
            // next.
            lastSeen[bitmap] = n;

            // Make prediction. We do this at points where:
            // (a) we have at least some idea of the period, and
            // (b) there are a whole number of periods *remaining* before rock 1e12, because we
            // only track the aggregate height increase per period.
            //
            // Note: predictions will take a few iterations to converge on the correct answer,
            // since we need time to find what the correct period actually is (as above).
            if(period > 0 && ((1000000000000 - n - 1) % period == 0))
            {
                long predictedTotalHeight = ((1000000000000 - n - 1) / period) * periodicIncrease + heightSoFar;
                
                Console.WriteLine("  Height increases " + periodicIncrease + " every " + period + " rocks => predicted total height = " + predictedTotalHeight);
                periodicIncrease = 0;
            }
        }
    }
}
