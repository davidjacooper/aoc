import scala.io.StdIn

object Day3_Part1
{
    def main(args: Array[String]): Unit =
    {
        println(run())
    }

    def run(): Int =
    {
        val line = StdIn.readLine()
        if(line == null) 
        {
            0
        }
        else
        {
            val halfWay = line.length() / 2
            val bits = makeBits(line.substring(0, halfWay))
            val matchedValue = findMatchedValue(bits, line.substring(halfWay))
            matchedValue + run()
        }
    }

    def makeBits(firstHalf: String): Long =
    {
        if(firstHalf.length() == 0)
        {
            0L
        }
        else
        {
            (1L << valueOf(firstHalf.charAt(0))) | makeBits(firstHalf.substring(1))
        }
    }

    def findMatchedValue(bits: Long, secondHalf: String): Int =
    {
        val value = valueOf(secondHalf.charAt(0))
        if((bits & (1L << value)) > 0)
        {
            value
        }
        else
        {
            findMatchedValue(bits, secondHalf.substring(1))
        }
    }

    def valueOf(ch: Char): Int =
        if(ch < 'a') ch - 'A' + 27 else ch - 'a' + 1
}

