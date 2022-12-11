       Identification division.
       Program-ID. Day4_Part1.
       
       Environment division.
       Input-output section.
       File-control.
       Select SYSIN assign to keyboard organization line sequential.
       
       Data division.
           File section.
               Fd sysin.
               01 input-line pic x(255).
               88 eof value high-values.
       
           Working-storage section.
               01 n-pairs  usage index.
               01 start1 pic 9(2).
               01 start2 pic 9(2).
               01 end1   pic 9(2).
               01 end2   pic 9(2).

       Procedure division.

            Open input sysin
            Move 0 to n-pairs
            
            Perform until 0 > 1
                Read sysin 
                    At end
                        Exit perform
                End-Read
                
                Unstring input-line
                    Delimited by "-" or ","
                    Into start1 end1 start2 end2
                End-Unstring
                
                If not ((end1 < start2) or (end2 < start1))
                    Add 1 to n-pairs
                End-If
            End-Perform
            
            Display n-pairs
           
            Close sysin
           
        Stop run.
