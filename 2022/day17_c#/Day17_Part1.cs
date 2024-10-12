using System;
using System.Collections;

class Day17_Part1
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
    
    static void PrintRockPile(ArrayList rockPile)
    {
        for(int i = rockPile.Count - 1; i >= 0; i--)
        {
            Console.Write("  |");
            for(int j = 6; j >= 0; j--)
            {
                if(((int)rockPile[i] & (1 << j)) > 0)
                {
                    Console.Write('#');
                }
                else
                {
                    Console.Write('.');
                }
            }
            Console.WriteLine('|');
        }
        Console.WriteLine("  +-------+\n");
    }
    
    static void Main(string[] args)
    {
        var directions = Console.ReadLine();
        int rockType = 0;
        int jetIndex = 0;
        
        var rockPile = new ArrayList();
        
        for(int n = 0; n < 2022; n++)
        {
            Console.WriteLine("Rock " + (n + 1));
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
                
                if(i >= rockPile.Count)
                {
                    rockPile.Add(rockRow);
                }
                else
                {
                    rockPile[i] = (int)rockPile[i] | rockRow;
                }
            }
        }
        
        PrintRockPile(rockPile);
        Console.WriteLine("Final height==" + rockPile.Count);
    }
}
