import scala.io.StdIn

object Day3_Part2
{
    def main(args: Array[String]): Unit =
    {
        println(run())
    }

    def run(): Int =
    {
        val line1 = StdIn.readLine()
        if(line1 == null) 
        {
            0
        }
        else
        {
            val line2 = StdIn.readLine()
            val line3 = StdIn.readLine()
            findMatchedValue(makeBits(line1) & makeBits(line2), line3) + run()
        }
    }

    def makeBits(line: String): Long =
    {
        if(line.length() == 0)
        {
            0L
        }
        else
        {
            (1L << valueOf(line.charAt(0))) | makeBits(line.substring(1))
        }
    }

    def findMatchedValue(bits: Long, line: String): Int =
    {
        val value = valueOf(line.charAt(0))
        if((bits & (1L << value)) > 0)
        {
            value
        }
        else
        {
            findMatchedValue(bits, line.substring(1))
        }
    }

    def valueOf(ch: Char): Int =
        if(ch < 'a') ch - 'A' + 27 else ch - 'a' + 1
}

