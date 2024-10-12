package main

import "fmt"
import "bufio"
import "os"
import "math/bits"

func main() {
    line, _ := bufio.NewReader(os.Stdin).ReadString('\n')

    for i := 13; i < len(line); i++ {
        var bitset uint32 = 0
        for j := i - 13; j <= i; j++ {
            bitset |= 1 << (line[j] - 'a')
        }
        if bits.OnesCount32(bitset) == 14 {
            fmt.Printf("%d\n", i + 1)
            break
        }
    }
}
